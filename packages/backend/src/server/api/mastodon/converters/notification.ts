import { ILocalUser, User } from "@/models/entities/user.js";
import { Notification } from "@/models/entities/notification.js";
import { notificationTypes } from "@/types.js";
import { UserConverter } from "@/server/api/mastodon/converters/user.js";
import { UserHelpers } from "@/server/api/mastodon/helpers/user.js";
import { awaitAll } from "@/prelude/await-all.js";
import { NoteConverter } from "@/server/api/mastodon/converters/note.js";
import { getNote } from "@/server/api/common/getters.js";
import {
	getStubMastoContext,
	MastoContext,
} from "@/server/api/mastodon/index.js";
import { Apps, Notifications } from "@/models/index.js";
import isQuote from "@/misc/is-quote.js";
import { unique } from "@/prelude/array.js";
import { Note } from "@/models/entities/note.js";
import { SwSubscription } from "@/models/entities/sw-subscription.js";
import { fetchMeta } from "@/misc/fetch-meta.js";
import type { pushNotificationsTypes } from "@/services/push-notification.js";
import { getNoteSummary } from "@/misc/get-note-summary.js";

type NotificationType = (typeof notificationTypes)[number];

export class NotificationConverter {
	public static async encode(
		notification: Notification,
		ctx: MastoContext,
	): Promise<MastodonEntity.Notification> {
		const localUser = ctx.user as ILocalUser;
		if (notification.notifieeId !== localUser.id)
			throw new Error("User is not recipient of notification");

		const account = notification.notifierId
			? UserHelpers.getUserCached(notification.notifierId, ctx).then((p) =>
					UserConverter.encode(p, ctx),
			  )
			: UserConverter.encode(localUser, ctx);

		let result = {
			id: notification.id,
			account: account,
			created_at: notification.createdAt.toISOString(),
			type: this.encodeNotificationType(notification.type),
		};

		const note =
			notification.note ??
			(notification.noteId
				? await getNote(notification.noteId, localUser)
				: null);

		if (note) {
			const isPureRenote = note.renoteId !== null && !isQuote(note);
			const encodedNote = isPureRenote
				? getNote(note.renoteId!, localUser).then((note) =>
						NoteConverter.encode(note, ctx),
				  )
				: NoteConverter.encode(note, ctx);
			result = Object.assign(result, {
				status: encodedNote,
			});
			if (result.type === "poll") {
				result = Object.assign(result, {
					account: encodedNote.then((p) => p.account),
				});
			}
			if (notification.reaction) {
				//FIXME: Implement reactions;
			}
		}
		return awaitAll(result);
	}

	public static async encodeMany(
		notifications: Notification[],
		ctx: MastoContext,
	): Promise<MastodonEntity.Notification[]> {
		await this.aggregateData(notifications, ctx);
		const encoded = notifications.map((u) => this.encode(u, ctx));
		return Promise.all(encoded).then(
			(p) => p.filter((n) => n !== null) as MastodonEntity.Notification[],
		);
	}

	private static async aggregateData(
		notifications: Notification[],
		ctx: MastoContext,
	): Promise<void> {
		if (notifications.length === 0) return;
		const notes = unique(
			notifications.filter((p) => p.note != null).map((n) => n.note as Note),
		);
		const users = unique(
			notifications
				.filter((p) => p.notifier != null)
				.map((n) => n.notifier as User)
				.concat(
					notifications
						.filter((p) => p.notifiee != null)
						.map((n) => n.notifiee as User),
				),
		);
		await NoteConverter.aggregateData(notes, ctx);
		await UserConverter.aggregateData(users, ctx);
	}

	private static encodeNotificationType(
		t: NotificationType,
	): MastodonEntity.NotificationType {
		//FIXME: Implement custom notification for followRequestAccepted
		//FIXME: Implement mastodon notification type 'update' on misskey side
		switch (t) {
			case "follow":
				return "follow";
			case "mention":
			case "reply":
				return "mention";
			case "renote":
				return "reblog";
			case "quote":
				return "reblog";
			case "reaction":
				return "favourite";
			case "pollEnded":
				return "poll";
			case "receiveFollowRequest":
				return "follow_request";
			case "followRequestAccepted":
			case "pollVote":
			case "groupInvited":
			case "app":
				throw new Error(`Notification type ${t} not supported`);
		}
	}

	public static async encodeNotificationTypeOrDefault(
		t: NotificationType,
	): Promise<string> {
		try {
			return this.encodeNotificationType(t);
		} catch (e) {
			return t;
		}
	}

	public static async encodeEvent(
		target: Notification["id"],
		user: ILocalUser,
		filterContext?: string,
	): Promise<MastodonEntity.Notification | null> {
		const ctx = getStubMastoContext(user, filterContext);
		const notification = await Notifications.findOneByOrFail({ id: target });
		return this.encode(notification, ctx).catch((_) => null);
	}

	public static async encodeSubscription(
		subscription: SwSubscription,
		ctx: MastoContext,
	): Promise<MastodonEntity.PushSubscription> {
		const instance = await fetchMeta(true);
		return {
			id: subscription.id,
			endpoint: subscription.endpoint,
			server_key: instance.swPublicKey ?? "",
			alerts: {
				// FIXME: Implement alerts
				follow: true,
				favourite: true,
				mention: true,
				reblog: true,
				poll: true,
				status: true,
			},
		};
	}

	public static async encodePushNotificationPayload<
		T extends keyof pushNotificationsTypes,
	>(
		subscription: SwSubscription,
		type: T,
		body: pushNotificationsTypes[T],
	): Promise<Partial<MastodonEntity.NotificationPayload>> {
		if (!subscription.appAccessToken) return {};
		const app = subscription.appAccessToken.appId
			? await Apps.findOneBy({ id: subscription.appAccessToken.appId })
			: null;
		const access_token = subscription.appAccessToken.token;
		let preferred_locale = "en";
		let notification_id = "";
		let notification_type = "others";
		let icon: string | undefined = undefined;
		let title = `New ${type} notification`;
		let description = "";
		if (type === "notification") {
			const notificationBody = body as pushNotificationsTypes["notification"];
			preferred_locale = notificationBody.note?.lang ?? preferred_locale;
			notification_id = notificationBody.id;
			notification_type = await this.encodeNotificationTypeOrDefault(
				notificationBody.type,
			);
			const effectiveNote =
				notificationBody.note?.renote ?? notificationBody.note;

			icon =
				notificationBody.user?.avatarUrl ??
				notificationBody.note?.user.avatarUrl ??
				notificationBody.icon ??
				undefined;
			const displayName =
				notificationBody.user?.name ||
				(notificationBody.user?.host &&
					`@${notificationBody.user?.username}@${notificationBody.user?.host}`) ||
				(notificationBody.user?.username &&
					`@${notificationBody.user?.username}`) ||
				"Someone";

			switch (notificationBody.type) {
				case "mention":
					title = `${displayName} mentioned you`;
					break;
				case "reply":
					title = `${displayName} replied to you`;
					break;
				case "renote":
					title = `${displayName} boosted your note`;
					break;
				case "quote":
					title = `${displayName} quoted your note`;
					break;
				case "reaction":
					title = `${displayName} reacted to your note`;
					break;
				case "pollVote":
					title = `${displayName} voted ${
						notificationBody.note?.poll.choices[notificationBody.choice || 0]
							?.text
					}`;
					break;
				case "pollEnded":
					title = `${displayName} closed a poll`;
					break;
				case "followRequestAccepted":
					title = `${displayName} accepted your follow request`;
					break;
				case "groupInvited":
					title = `${displayName} invited you to ${notificationBody.invitation.group.name}`;
					break;
				case "app":
					title = `${notificationBody.header}`;
					break;
				default:
					title = `New ${type} (${notificationBody.type}) notification`;
			}
			description =
				(effectiveNote && getNoteSummary(effectiveNote)) ||
				notificationBody.body ||
				"";
		} else if (type === "unreadMessagingMessage") {
			const notificationBody =
				body as pushNotificationsTypes["unreadMessagingMessage"];
			notification_id = notificationBody.id;
			notification_type = "messaging";
			icon = notificationBody.user?.avatarUrl ?? "";
			description = notificationBody.text || "";
		}

		// App-specific compats
		if (
			app?.name === "Mastodon for Android" ||
			app?.name === "Megalodon" ||
			app?.name === "Moshidon"
		) {
			// Mastodon for Android (and forks) require notification_id to be convertible to a long int, but never use it.
			notification_id = new Date().getTime().toString();
		}

		return {
			access_token,
			preferred_locale,
			notification_id,
			notification_type,
			icon,
			title,
			body: description,
		};
	}
}
