import push from "web-push";
import { config } from "@/config.js";
import { SwSubscriptions, UserProfiles } from "@/models/index.js";
import { fetchMeta, getNoteSummary } from "backend-rs";
import type { Packed } from "@/misc/schema.js";
import { NotificationConverter } from "@/server/api/mastodon/converters/notification.js";

// Defined also packages/sw/types.ts#L14-L21
export type pushNotificationsTypes = {
	notification: Packed<"Notification">;
	unreadMessagingMessage: Packed<"MessagingMessage">;
	readNotifications: { notificationIds: string[] };
	readAllNotifications: undefined;
	readAllMessagingMessages: undefined;
	readAllMessagingMessagesOfARoom: { userId: string } | { groupId: string };
};

// プッシュメッセージサーバーには文字数制限があるため、内容を削減します
function truncateNotification(notification: Packed<"Notification">): any {
	if (notification.note != null) {
		return {
			...notification,
			note: {
				...notification.note,
				// replace the text with summary
				text: getNoteSummary(
					notification.type === "renote" && notification.note.renote != null
						? notification.note.renote
						: notification.note,
				),

				cw: undefined,
				reply: undefined,
				renote: undefined,
				user: undefined as any, // 通知を受け取ったユーザーである場合が多いのでこれも捨てる
			},
		};
	}

	return notification;
}

export async function pushNotification<T extends keyof pushNotificationsTypes>(
	userId: string,
	type: T,
	body: pushNotificationsTypes[T],
) {
	const meta = await fetchMeta(true);

	if (
		!meta.enableServiceWorker ||
		meta.swPublicKey == null ||
		meta.swPrivateKey == null
	)
		return;

	// アプリケーションの連絡先と、サーバーサイドの鍵ペアの情報を登録
	push.setVapidDetails(config.url, meta.swPublicKey, meta.swPrivateKey);

	// Fetch
	const subscriptions = await SwSubscriptions.find({
		where: { userId: userId },
		relations: ["appAccessToken"],
	});
	const userProfile = await UserProfiles.findOneBy({ userId: userId });

	for (const subscription of subscriptions) {
		if (subscription.appAccessTokenId) {
			if (
				type === "readNotifications" ||
				type === "readAllNotifications" ||
				type === "readAllMessagingMessages" ||
				type === "readAllMessagingMessagesOfARoom"
			) {
				continue;
			}
			if (type === "notification") {
				const notificatonType = await NotificationConverter.encodeNotificationTypeOrDefault((body as Packed<"Notification">).type);
				if (!subscription.subscriptionTypes.includes(notificatonType)) {
					continue;
				}
			}
		}

		const pushSubscription = {
			endpoint: subscription.endpoint,
			keys: {
				auth: subscription.auth,
				p256dh: subscription.publickey,
			},
		};
		
		let notificationPayload: Record<string, unknown>;

		if (subscription.appAccessToken) {
			notificationPayload = await NotificationConverter.encodePushNotificationPayload(
				subscription,
				type,
				body,
				userProfile?.lang ?? undefined,
			);
		} else {
			notificationPayload = {
				type,
				body:
					type === "notification"
						? truncateNotification(body as Packed<"Notification">)
						: body,
				userId,
				dateTime: Date.now(),
			};
		}
				
		console.log(
			"Push notification, pushSubscription:",
			pushSubscription,
			", notificationPayload:",
			notificationPayload,
			", body:",
			body,
			", Stringify payload:",
			JSON.stringify(notificationPayload),
		);
		push
			.sendNotification(pushSubscription, JSON.stringify(notificationPayload), {
				proxy: config.proxy,
				...(subscription.appAccessToken ? { contentEncoding: "aesgcm" } : {}),
			})
			.then((result) => {
				console.log(
					"Push notification, pushSubscription:",
					pushSubscription,
					", result:",
					result,
				);
				return result;
			})
			.catch((err: any) => {
				console.log(
					"Push notification, pushSubscription:",
					pushSubscription,
					", error:",
					err,
				);
				if (err.statusCode === 410) {
					SwSubscriptions.delete({
						userId: userId,
						endpoint: subscription.endpoint,
						auth: subscription.auth,
						publickey: subscription.publickey,
					});
				}
			});
	}
}
