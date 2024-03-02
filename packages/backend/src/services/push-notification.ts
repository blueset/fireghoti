import push from "web-push";
import config from "@/config/index.js";
import { SwSubscriptions } from "@/models/index.js";
import { fetchMeta } from "@/misc/fetch-meta.js";
import type { Packed } from "@/misc/schema.js";
import { getNoteSummary } from "@/misc/get-note-summary.js";
import { convertId, IdConvertType as IdType } from "backend-rs";
import { Converter } from "megalodon";

// Defined also packages/sw/types.ts#L14-L21
type pushNotificationsTypes = {
	notification: Packed<"Notification">;
	unreadMessagingMessage: Packed<"MessagingMessage">;
	readNotifications: { notificationIds: string[] };
	readAllNotifications: undefined;
	readAllMessagingMessages: undefined;
	readAllMessagingMessagesOfARoom: { userId: string } | { groupId: string };
};

// プッシュメッセージサーバーには文字数制限があるため、内容を削減します
function truncateNotification(notification: Packed<"Notification">): any {
	if (notification.note) {
		return {
			...notification,
			note: {
				...notification.note,
				// textをgetNoteSummaryしたものに置き換える
				text: getNoteSummary(
					notification.type === "renote"
						? (notification.note.renote as Packed<"Note">)
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
	const meta = await fetchMeta();

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

	for (const subscription of subscriptions) {
		if (
			[
				"readNotifications",
				"readAllNotifications",
				"readAllMessagingMessages",
				"readAllMessagingMessagesOfARoom",
			].includes(type) &&
			!subscription.sendReadMessage
		)
			continue;

		const pushSubscription = {
			endpoint: subscription.endpoint,
			keys: {
				auth: subscription.auth,
				p256dh: subscription.publickey,
			},
		};
		const converter = new Converter("");
		const notificationBody = body as Packed<"Notification">;
		const displayName = (type === "notification" && (
			notificationBody.user?.name ||
			(notificationBody.user?.host && `@${notificationBody.user?.username}@${notificationBody.user?.host}`) ||
			(notificationBody.user?.username && `@${notificationBody.user?.username}`) 
		)) || "Someone";
		const notificationPayload = subscription.appAccessToken ? {
			// Push notification payload for Mastodon
			access_token: subscription.appAccessToken.token,
			preferred_locale: (type === "notification" && notificationBody.note?.lang) || "en",
			notification_id: 
			convertId((type === "notification" && notificationBody.id) || (type === "unreadMessagingMessage" && (body as Packed<"MessagingMessage">).id) || "", IdType.MastodonId),
			// (type === "notification" && notificationBody.id) || (type === "unreadMessagingMessage" && (body as Packed<"MessagingMessage">).id) || "",
			notification_type: 
				(type === "notification" && 
					converter.decodeNotificationType(notificationBody.type)) || 
				(type === "unreadMessagingMessage" && "messaging") || 
				"others",
			icon: (type === "notification" && 
					(
						notificationBody.user?.avatarUrl || 
						notificationBody.note?.user.avatarUrl ||
						notificationBody.icon
					)
				) || 
				(type === "unreadMessagingMessage" && (body as Packed<"MessagingMessage">).user?.avatarUrl) || "",
			title: (type === "notification" && (
				(notificationBody.type === "mention" && `${displayName} mentioned you`) ||
				(notificationBody.type === "reply" && `${displayName} replied you`) ||
				(notificationBody.type === "renote" && `${displayName} boosted your note`) ||
				(notificationBody.type === "quote" && `${displayName} quoted your note`) ||
				(notificationBody.type === "reaction" && `${displayName} reacted ${notificationBody.reaction}`) ||
				(notificationBody.type === "pollVote" && `${displayName} voted ${notificationBody.note?.poll.choices[notificationBody.choice || 0]?.text}`) ||
				(notificationBody.type === "pollEnded" && `${displayName} closed a poll`) ||
				(notificationBody.type === "groupInvited" && `${displayName} invited you to ${notificationBody.invitation.group.name}`) ||
				(notificationBody.type === "app" && notificationBody.header)
			)) || `New ${type} notification`,
			body: (type === "notification" && (body as Packed<"Notification">).note?.text) || 
				(type === "unreadMessagingMessage" && (body as Packed<"MessagingMessage">).text) ||notificationBody.body || "",
		} : {
			type,
			body:
				type === "notification"
					? truncateNotification(body as Packed<"Notification">)
					: body,
			userId,
			dateTime: new Date().getTime(),
		};

		console.log("Push notification, pushSubscription:", pushSubscription, ", notificationPayload:", notificationPayload, ", body:", body, ", Stringify payload:", JSON.stringify(notificationPayload));
		push
			.sendNotification(
				pushSubscription,
				JSON.stringify(notificationPayload),
				{
					proxy: config.proxy,
				},
			)
			.catch((err: any) => {
				//swLogger.info(err.statusCode);
				//swLogger.info(err.headers);
				//swLogger.info(err.body);

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
