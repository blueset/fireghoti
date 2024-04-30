import type { entities } from "firefish-js";

export type FoldableNotification =
	| entities.RenoteNotification
	| entities.ReactionNotification;

interface Fold<T extends FoldableNotification> {
	id: string;
	type: T["type"];
	createdAt: T["createdAt"];
	note: T["note"];
	folded: true;
	userIds: entities.User["id"][];
	users: entities.User[];
	notifications: T[];
}

export type RenoteNotificationFolded = Fold<entities.RenoteNotification>;

export type ReactionNotificationFolded = Fold<entities.ReactionNotification> & {
	reaction: string;
};

export type GetNotificationFoldedType<T extends FoldableNotification> =
	T["type"] extends "renote"
		? RenoteNotificationFolded
		: ReactionNotificationFolded;

export type NotificationFolded =
	| RenoteNotificationFolded
	| ReactionNotificationFolded;
