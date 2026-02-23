import type Router from "@koa/router";
import { limitToInt, normalizeUrlQuery } from "./timeline.js";
import { NotificationHelpers } from "@/server/api/mastodon/helpers/notification.js";
import { NotificationConverter } from "@/server/api/mastodon/converters/notification.js";
import { auth } from "@/server/api/mastodon/middleware/auth.js";
import { filterContext } from "@/server/api/mastodon/middleware/filter-context.js";
import { MastoApiError } from "@/server/api/mastodon/middleware/catch-errors.js";

/** Permissive notification policy (accept everything — no filtering implemented) */
const PERMISSIVE_NOTIFICATION_POLICY: MastodonEntity.NotificationPolicy = {
	for_not_following: "accept",
	for_not_followers: "accept",
	for_new_accounts: "accept",
	for_private_mentions: "accept",
	for_limited_accounts: "accept",
	summary: {
		pending_requests_count: 0,
		pending_notifications_count: 0,
	},
};

export function setupEndpointsNotifications(router: Router): void {
	// =====================
	// v1 notification endpoints
	// =====================

	router.get(
		"/v1/notifications",
		auth(true, ["read:notifications"]),
		filterContext("notifications"),
		async (ctx) => {
			const args = normalizeUrlQuery(limitToInt(ctx.query), [
				"types[]",
				"exclude_types[]",
			]);
			const res = await NotificationHelpers.getNotifications(
				args.max_id,
				args.since_id,
				args.min_id,
				args.limit,
				args["types[]"],
				args["exclude_types[]"],
				args.account_id,
				ctx,
			);
			ctx.body = await NotificationConverter.encodeMany(res, ctx);
		},
	);

	router.get(
		"/v1/notifications/unread_count",
		auth(true, ["read:notifications"]),
		async (ctx) => {
			const args = normalizeUrlQuery(limitToInt(ctx.query), [
				"types[]",
				"exclude_types[]",
			]);
			const count = await NotificationHelpers.getUnreadNotificationCount(
				args.limit ?? 100,
				args["types[]"],
				args["exclude_types[]"],
				args.account_id,
				ctx,
			);
			ctx.body = { count };
		},
	);

	// Notification requests stubs (no filtering policy → always empty)
	router.get(
		"/v1/notifications/requests",
		auth(true, ["read:notifications"]),
		async (ctx) => {
			ctx.body = [];
		},
	);

	router.get(
		"/v1/notifications/requests/merged",
		auth(true, ["read:notifications"]),
		async (ctx) => {
			ctx.body = { merged: true };
		},
	);

	router.post(
		"/v1/notifications/requests/accept",
		auth(true, ["write:notifications"]),
		async (ctx) => {
			ctx.status = 501;
			ctx.body = { error: "Not implemented" };
		},
	);

	router.post(
		"/v1/notifications/requests/dismiss",
		auth(true, ["write:notifications"]),
		async (ctx) => {
			ctx.status = 501;
			ctx.body = { error: "Not implemented" };
		},
	);

	router.get(
		"/v1/notifications/requests/:id",
		auth(true, ["read:notifications"]),
		async (ctx) => {
			ctx.status = 404;
			ctx.body = { error: "Record not found" };
		},
	);

	router.post(
		"/v1/notifications/requests/:id/accept",
		auth(true, ["write:notifications"]),
		async (ctx) => {
			ctx.status = 501;
			ctx.body = { error: "Not implemented" };
		},
	);

	router.post(
		"/v1/notifications/requests/:id/dismiss",
		auth(true, ["write:notifications"]),
		async (ctx) => {
			ctx.status = 501;
			ctx.body = { error: "Not implemented" };
		},
	);

	router.get(
		"/v1/notifications/:id",
		auth(true, ["read:notifications"]),
		filterContext("notifications"),
		async (ctx) => {
			const notification = await NotificationHelpers.getNotificationOr404(
				ctx.params.id,
				ctx,
			);
			ctx.body = await NotificationConverter.encode(notification, ctx);
		},
	);

	router.post(
		"/v1/notifications/clear",
		auth(true, ["write:notifications"]),
		async (ctx) => {
			await NotificationHelpers.clearAllNotifications(ctx);
			ctx.body = {};
		},
	);

	router.post(
		"/v1/notifications/:id/dismiss",
		auth(true, ["write:notifications"]),
		async (ctx) => {
			const notification = await NotificationHelpers.getNotificationOr404(
				ctx.params.id,
				ctx,
			);
			await NotificationHelpers.dismissNotification(notification.id, ctx);
			ctx.body = {};
		},
	);

	router.get(
		"/v2/notifications",
		auth(true, ["read:notifications"]),
		filterContext("notifications"),
		async (ctx) => {
			const args = normalizeUrlQuery(limitToInt(ctx.query), [
				"types[]",
				"exclude_types[]",
				"grouped_types[]",
			]);
			const groups = await NotificationHelpers.getGroupedNotifications(
				args.max_id,
				args.since_id,
				args.min_id,
				args.limit,
				args["types[]"],
				args["exclude_types[]"],
				args["grouped_types[]"],
				args.account_id,
				ctx,
			);
			ctx.body = await NotificationConverter.encodeGroupedResults(groups, ctx);
		},
	);

	// Must be registered before "/v2/notifications/:group_key" so it is not
	// shadowed by the group-key route.
	router.get(
		"/v2/notifications/unread_count",
		auth(true, ["read:notifications"]),
		async (ctx) => {
			const args = normalizeUrlQuery(limitToInt(ctx.query), [
				"types[]",
				"exclude_types[]",
				"grouped_types[]",
			]);
			const count = await NotificationHelpers.getGroupedUnreadCount(
				args["types[]"],
				args["exclude_types[]"],
				args["grouped_types[]"],
				args.account_id,
				args.limit,
				ctx,
			);
			ctx.body = { count };
		},
	);

	// Notification policy stubs
	router.get(
		"/v2/notifications/policy",
		auth(true, ["read:notifications"]),
		async (ctx) => {
			ctx.body = PERMISSIVE_NOTIFICATION_POLICY;
		},
	);

	router.patch(
		"/v2/notifications/policy",
		auth(true, ["write:notifications"]),
		async (ctx) => {
			ctx.status = 501;
			ctx.body = { error: "Not implemented" };
		},
	);

	router.get(
		"/v2/notifications/:group_key/accounts",
		auth(true, ["read:notifications"]),
		async (ctx) => {
			const members = await NotificationHelpers.getNotificationsForGroupKey(
				ctx.params.group_key,
				ctx,
			);
			ctx.body = await NotificationConverter.encodeGroupAccounts(members, ctx);
		},
	);

	router.post(
		"/v2/notifications/:group_key/dismiss",
		auth(true, ["write:notifications"]),
		async (ctx) => {
			await NotificationHelpers.dismissGroup(ctx.params.group_key, ctx);
			ctx.body = {};
		},
	);

	router.get(
		"/v2/notifications/:group_key",
		auth(true, ["read:notifications"]),
		filterContext("notifications"),
		async (ctx) => {
			const members = await NotificationHelpers.getNotificationsForGroupKey(
				ctx.params.group_key,
				ctx,
			);
			if (members.length === 0) throw new MastoApiError(404);
			const totalCounts = new Map<string, number>([
				[ctx.params.group_key, members.length],
			]);
			ctx.body = await NotificationConverter.encodeGroupedResults(
				[{ groupKey: ctx.params.group_key, members }],
				ctx,
				totalCounts,
			);
		},
	);

	router.post(
		"/v1/conversations/:id/read",
		auth(true, ["write:conversations"]),
		async (ctx, _reply) => {
			await NotificationHelpers.markConversationAsRead(ctx.params.id, ctx);
			ctx.body = {};
		},
	);

	router.get("/v1/push/subscription", auth(true, ["push"]), async (ctx) => {
		const subscription =
			await NotificationHelpers.getPushSubscriptionOr404(ctx);
		ctx.body = await NotificationConverter.encodeSubscription(
			subscription,
			ctx,
		);
	});

	router.post("/v1/push/subscription", auth(true, ["push"]), async (ctx) => {
		const subscription = await NotificationHelpers.setPushSubscription(ctx);
		ctx.body = await NotificationConverter.encodeSubscription(
			subscription,
			ctx,
		);
	});

	router.delete("/v1/push/subscription", auth(true, ["push"]), async (ctx) => {
		await NotificationHelpers.deletePushSubscription(ctx);
		ctx.body = {};
	});

	router.put("/v1/push/subscription", auth(true, ["push"]), async (ctx) => {
		const subscription =
			await NotificationHelpers.getPushSubscriptionOr404(ctx);
		await NotificationHelpers.putPushSubscription(subscription, ctx);
		ctx.body = await NotificationConverter.encodeSubscription(
			subscription,
			ctx,
		);
	});
}
