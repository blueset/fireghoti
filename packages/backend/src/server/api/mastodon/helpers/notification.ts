import type { ILocalUser } from "@/models/entities/user.js";
import { Notes, Notifications, SwSubscriptions } from "@/models/index.js";
import { PaginationHelpers } from "@/server/api/mastodon/helpers/pagination.js";
import type { Notification } from "@/models/entities/notification.js";
import { MastoApiError } from "@/server/api/mastodon/middleware/catch-errors.js";
import type { MastoContext } from "@/server/api/mastodon/index.js";
import type { SwSubscription } from "@/models/entities/sw-subscription.js";
import { In } from "typeorm";

/**
 * Normalize object arguments from query string.
 *
 * @example
 * ```ts
 * normalizeObjectArgs({
 *    "subscription[endpoint]": "https://example.com",
 *    "subscription[keys][p256dh]": "key",
 *    "subscription[keys][auth]": "auth"
 * });
 * // { subscription: { endpoint: "https://example.com", keys: { p256dh: "key", auth: "auth" } } }
 * ```
 */
function normalizeObjectArgs(q: Record<string, string>) {
	const dict: Record<string, any> = {};

	for (const k in q) {
		if (k.endsWith("]")) {
			const segments = k.split("[").map((p) => p.replace(/]$/g, ""));
			let d = dict;
			for (let i = 0; i < segments.length - 1; i++) {
				if (!(segments[i] in d)) d[segments[i]] = {};
				d = d[segments[i]];
			}
			d[segments[segments.length - 1]] = q[k];
		} else {
			dict[k] = q[k];
		}
	}

	return dict;
}

export class NotificationHelpers {
	public static async getNotifications(
		maxId: string | undefined,
		sinceId: string | undefined,
		minId: string | undefined,
		limit = 40,
		types: string[] | undefined,
		excludeTypes: string[] | undefined,
		accountId: string | undefined,
		ctx: MastoContext,
	): Promise<Notification[]> {
		if (limit > 80) limit = 80;

		const user = ctx.user as ILocalUser;
		let requestedTypes = types
			? this.decodeTypes(types)
			: [
					"follow",
					"mention",
					"reply",
					"renote",
					"quote",
					"reaction",
					"pollEnded",
					"receiveFollowRequest",
				];

		if (excludeTypes) {
			const excludedTypes = this.decodeTypes(excludeTypes);
			requestedTypes = requestedTypes.filter((p) => !excludedTypes.includes(p));
		}

		if (!requestedTypes.length) return [];

		const query = PaginationHelpers.makePaginationQuery(
			Notifications.createQueryBuilder("notification"),
			sinceId,
			maxId,
			minId,
		)
			.andWhere("notification.notifieeId = :userId", { userId: user.id })
			.andWhere("notification.type IN (:...types)", { types: requestedTypes });

		if (accountId !== undefined)
			query.andWhere("notification.notifierId = :notifierId", {
				notifierId: accountId,
			});

		query
			.leftJoinAndSelect("notification.note", "note")
			.leftJoinAndSelect("notification.notifier", "notifier")
			.leftJoinAndSelect("notification.notifiee", "notifiee");

		return PaginationHelpers.execQueryLinkPagination(
			query,
			limit,
			minId !== undefined,
			ctx,
		);
	}

	public static async getNotification(
		id: string,
		ctx: MastoContext,
	): Promise<Notification | null> {
		const user = ctx.user as ILocalUser;
		return Notifications.findOneBy({ id: id, notifieeId: user.id });
	}

	public static async getNotificationOr404(
		id: string,
		ctx: MastoContext,
	): Promise<Notification> {
		return this.getNotification(id, ctx).then((p) => {
			if (p) return p;
			throw new MastoApiError(404);
		});
	}

	public static async dismissNotification(
		id: string,
		ctx: MastoContext,
	): Promise<void> {
		const user = ctx.user as ILocalUser;
		await Notifications.update(
			{ id: id, notifieeId: user.id },
			{ isRead: true },
		);
	}

	public static async clearAllNotifications(ctx: MastoContext): Promise<void> {
		const user = ctx.user as ILocalUser;
		await Notifications.update({ notifieeId: user.id }, { isRead: true });
	}

	public static async markConversationAsRead(
		id: string,
		ctx: MastoContext,
	): Promise<void> {
		const user = ctx.user as ILocalUser;
		const notesQuery = Notes.createQueryBuilder("note")
			.select("note.id")
			.andWhere("COALESCE(note.threadId, note.id) = :conversationId");

		await Notifications.createQueryBuilder("notification")
			.where(`notification."noteId" IN (${notesQuery.getQuery()})`)
			.andWhere(`notification."notifieeId" = :userId`)
			.andWhere(`notification."isRead" = FALSE`)
			.andWhere("notification.type IN (:...types)")
			.setParameter("userId", user.id)
			.setParameter("conversationId", id)
			.setParameter("types", ["reply", "mention"])
			.update()
			.set({ isRead: true })
			.execute();
	}

	/**
	 * Compute a Mastodon grouped-notification `group_key` for a notification
	 * without persisting it. The scheme is reversible (see
	 * {@link getNotificationsForGroupKey}) and treats `group_key` as opaque per
	 * the Mastodon spec.
	 *
	 * Grouped types (favourite, reblog, follow) collapse into a shared key;
	 * everything else (mentions, polls, follow requests, quotes, ...) becomes
	 * `ungrouped-{notificationId}`.
	 *
	 * @param notification A notification with its `note` relation loaded.
	 * @param groupedTypes Mastodon types the client wants grouped. When a
	 *   notification maps to a type not in this list it is returned ungrouped.
	 */
	private static computeGroupKey(
		notification: Notification,
		groupedTypes?: string[],
	): string {
		switch (notification.type) {
			case "reaction":
				if (notification.noteId == null) break;
				if (groupedTypes && !groupedTypes.includes("favourite")) break;
				return `favourite-${notification.noteId}`;
			case "renote":
				if (notification.note?.renoteId == null) break;
				if (groupedTypes && !groupedTypes.includes("reblog")) break;
				return `reblog-${notification.note.renoteId}`;
			case "follow":
				if (groupedTypes && !groupedTypes.includes("follow")) break;
				return "follow";
		}
		return `ungrouped-${notification.id}`;
	}

	private static groupNotifications(
		notifications: Notification[],
		groupedTypes?: string[],
	): { groupKey: string; members: Notification[] }[] {
		const groups = new Map<string, Notification[]>();
		const order: string[] = [];
		for (const notification of notifications) {
			const key = this.computeGroupKey(notification, groupedTypes);
			const existing = groups.get(key);
			if (existing) {
				existing.push(notification);
			} else {
				groups.set(key, [notification]);
				order.push(key);
			}
		}
		return order.map((key) => ({
			groupKey: key,
			members: groups.get(key) as Notification[],
		}));
	}

	/**
	 * Fetch a page of notifications and group them in-memory for the Mastodon
	 * `/api/v2/notifications` endpoint. Pagination (and the `Link` header) is
	 * driven by the underlying raw notification IDs, so a page may contain fewer
	 * groups than `limit`.
	 */
	public static async getGroupedNotifications(
		maxId: string | undefined,
		sinceId: string | undefined,
		minId: string | undefined,
		limit = 40,
		types: string[] | undefined,
		excludeTypes: string[] | undefined,
		groupedTypes: string[] | undefined,
		accountId: string | undefined,
		ctx: MastoContext,
	): Promise<{ groupKey: string; members: Notification[] }[]> {
		const notifications = await this.getNotifications(
			maxId,
			sinceId,
			minId,
			limit,
			types,
			excludeTypes,
			accountId,
			ctx,
		);
		return this.groupNotifications(notifications, groupedTypes);
	}

	/**
	 * Resolve all notifications belonging to a `group_key` produced by
	 * {@link computeGroupKey}, for the single-group, accounts and dismiss
	 * endpoints. Returns an empty array for unknown keys.
	 */
	public static async getNotificationsForGroupKey(
		groupKey: string,
		ctx: MastoContext,
	): Promise<Notification[]> {
		const user = ctx.user as ILocalUser;
		const query = Notifications.createQueryBuilder("notification")
			.leftJoinAndSelect("notification.note", "note")
			.leftJoinAndSelect("notification.notifier", "notifier")
			.leftJoinAndSelect("notification.notifiee", "notifiee")
			.andWhere("notification.notifieeId = :userId", { userId: user.id })
			.orderBy("notification.id", "DESC");

		if (groupKey.startsWith("ungrouped-")) {
			query.andWhere("notification.id = :gkId", {
				gkId: groupKey.slice("ungrouped-".length),
			});
		} else if (groupKey === "follow") {
			query.andWhere("notification.type = :gkType", { gkType: "follow" });
		} else if (groupKey.startsWith("favourite-")) {
			query
				.andWhere("notification.type = :gkType", { gkType: "reaction" })
				.andWhere("notification.noteId = :gkNoteId", {
					gkNoteId: groupKey.slice("favourite-".length),
				});
		} else if (groupKey.startsWith("reblog-")) {
			query
				.andWhere("notification.type = :gkType", { gkType: "renote" })
				.andWhere("note.renoteId = :gkNoteId", {
					gkNoteId: groupKey.slice("reblog-".length),
				});
		} else {
			return [];
		}

		return query.getMany();
	}

	public static async dismissGroup(
		groupKey: string,
		ctx: MastoContext,
	): Promise<void> {
		const user = ctx.user as ILocalUser;
		const notifications = await this.getNotificationsForGroupKey(groupKey, ctx);
		const ids = notifications.map((n) => n.id);
		if (ids.length === 0) return;
		await Notifications.update(
			{ id: In(ids), notifieeId: user.id },
			{ isRead: true },
		);
	}

	/**
	 * Approximate (capped) count of unread notification groups for the Mastodon
	 * `/api/v2/notifications/unread_count` endpoint. Scans at most 1000 unread
	 * notifications (newest first) and counts distinct group keys up to `limit`.
	 */
	public static async getGroupedUnreadCount(
		types: string[] | undefined,
		excludeTypes: string[] | undefined,
		groupedTypes: string[] | undefined,
		accountId: string | undefined,
		limit = 100,
		ctx: MastoContext,
	): Promise<number> {
		if (limit > 1000) limit = 1000;

		const user = ctx.user as ILocalUser;
		let requestedTypes = types
			? this.decodeTypes(types)
			: [
					"follow",
					"mention",
					"reply",
					"renote",
					"quote",
					"reaction",
					"pollEnded",
					"receiveFollowRequest",
				];

		if (excludeTypes) {
			const excludedTypes = this.decodeTypes(excludeTypes);
			requestedTypes = requestedTypes.filter((p) => !excludedTypes.includes(p));
		}

		if (!requestedTypes.length) return 0;

		const query = Notifications.createQueryBuilder("notification")
			.leftJoinAndSelect("notification.note", "note")
			.andWhere("notification.notifieeId = :userId", { userId: user.id })
			.andWhere("notification.isRead = FALSE")
			.andWhere("notification.type IN (:...types)", { types: requestedTypes })
			.orderBy("notification.id", "DESC")
			.take(1000);

		if (accountId !== undefined)
			query.andWhere("notification.notifierId = :notifierId", {
				notifierId: accountId,
			});

		const notifications = await query.getMany();
		const keys = new Set<string>();
		for (const notification of notifications) {
			keys.add(this.computeGroupKey(notification, groupedTypes));
			if (keys.size >= limit) break;
		}
		return keys.size;
	}

	public static async getPushSubscription(
		ctx: MastoContext,
	): Promise<SwSubscription | null> {
		const user = ctx.user as ILocalUser;
		const tokenId = ctx.tokenId as string;
		const subscription = await SwSubscriptions.findOneBy({
			userId: user.id,
			appAccessTokenId: tokenId,
		});
		return subscription;
	}

	public static async getPushSubscriptionOr404(
		ctx: MastoContext,
	): Promise<SwSubscription> {
		const subscription = await this.getPushSubscription(ctx);
		if (subscription) return subscription;
		throw new MastoApiError(404);
	}

	public static async setPushSubscription(
		ctx: MastoContext,
	): Promise<SwSubscription> {
		const user = ctx.user as ILocalUser;
		const tokenId = ctx.tokenId as string;
		let body = ctx.request.body as any;
		if ("subscription[endpoint]" in body) {
			body = normalizeObjectArgs(body);
		}
		const subscription = body.subscription as {
			endpoint: string;
			keys: { p256dh: string; auth: string };
		};
		const alerts = body.data.alerts as Record<
			MastodonEntity.NotificationType,
			boolean
		>;

		const existing = await SwSubscriptions.findOneBy({
			userId: user.id,
			appAccessTokenId: tokenId,
		});

		const types = (
			Object.keys(alerts) as MastodonEntity.NotificationType[]
		).filter((k) => alerts[k]);

		if (existing) {
			await SwSubscriptions.update(
				{ userId: user.id, appAccessTokenId: tokenId },
				{
					endpoint: subscription.endpoint,
					publickey: subscription.keys.p256dh,
					auth: subscription.keys.auth,
					sendReadMessage: false,
					appAccessTokenId: tokenId,
					subscriptionTypes: types,
				},
			);
		} else {
			await SwSubscriptions.insert({
				id: tokenId,
				userId: user.id,
				createdAt: new Date(),
				endpoint: subscription.endpoint,
				publickey: subscription.keys.p256dh,
				auth: subscription.keys.auth,
				sendReadMessage: false,
				appAccessTokenId: tokenId,
				subscriptionTypes: types,
			});
		}
		return SwSubscriptions.findOneByOrFail({
			userId: user.id,
			appAccessTokenId: tokenId,
		});
	}

	public static async putPushSubscription(
		subscription: SwSubscription,
		ctx: MastoContext,
	) {
		let body = ctx.request.body as any;
		if ("data[alerts][follow]" in body) {
			body = normalizeObjectArgs(body);
		}
		const alerts = body.data.alerts as Record<
			MastodonEntity.NotificationType,
			boolean
		>;
		const types = subscription.subscriptionTypes;
		for (const type of Object.keys(
			alerts,
		) as MastodonEntity.NotificationType[]) {
			if (alerts[type]) {
				if (!types.includes(type)) {
					types.push(type);
				}
			} else {
				const index = types.indexOf(type);
				if (index !== -1) {
					types.splice(index, 1);
				}
			}
		}
		await SwSubscriptions.update(
			{
				userId: subscription.userId,
				appAccessTokenId: subscription.appAccessTokenId ?? undefined,
			},
			{
				subscriptionTypes: types,
			},
		);
	}

	public static async deletePushSubscription(ctx: MastoContext): Promise<void> {
		const user = ctx.user as ILocalUser;
		const tokenId = ctx.tokenId as string;
		await SwSubscriptions.delete({
			userId: user.id,
			appAccessTokenId: tokenId,
		});
	}

	private static decodeTypes(types: string[]) {
		const result: string[] = [];
		if (types.includes("follow")) result.push("follow");
		if (types.includes("mention")) result.push("mention", "reply");
		if (types.includes("reblog")) result.push("renote", "quote");
		if (types.includes("favourite")) result.push("reaction");
		if (types.includes("poll")) result.push("pollEnded");
		if (types.includes("follow_request")) result.push("receiveFollowRequest");
		return result;
	}
}
