/// <reference path="account.ts" />
/// <reference path="status.ts" />

namespace MastodonEntity {
	export type Notification = {
		account: Account;
		created_at: string;
		id: string;
		status?: Status;
		reaction?: Reaction;
		type: NotificationType;
	};

	export type NotificationGroup = {
		group_key: string;
		notifications_count: number;
		type: NotificationType;
		most_recent_notification_id: string;
		page_min_id?: string;
		page_max_id?: string;
		latest_page_notification_at?: string;
		sample_account_ids: string[];
		status_id?: string;
	};

	export type GroupedNotificationsResults = {
		accounts: Account[];
		partial_accounts?: PartialAccountWithAvatar[];
		statuses: Status[];
		notification_groups: NotificationGroup[];
	};

	export type NotificationType =
		| "mention"
		| "status"
		| "reblog"
		| "follow"
		| "follow_request"
		| "favourite"
		| "poll"
		| "update"
		| "admin.sign_up"
		| "admin.report"
		| "severed_relationships"
		| "moderation_warning"
		| "quote"
		| "quoted_update";

	export type PartialAccountWithAvatar = {
		id: string;
		acct: string;
		url: string;
		avatar: string;
		avatar_static: string;
		locked: boolean;
		bot: boolean;
	};

	export type NotificationPolicy = {
		for_not_following: string;
		for_not_followers: string;
		for_new_accounts: string;
		for_private_mentions: string;
		for_limited_accounts: string;
		summary: {
			pending_requests_count: number;
			pending_notifications_count: number;
		};
	};
}
