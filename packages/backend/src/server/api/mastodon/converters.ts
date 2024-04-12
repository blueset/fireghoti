import { Entity } from "megalodon";
import { toMastodonId } from "backend-rs";

function simpleConvert(data: any) {
	// copy the object to bypass weird pass by reference bugs
	const result = Object.assign({}, data);
	result.id = toMastodonId(data.id);
	return result;
}

export function convertAccount(account: Entity.Account) {
	return simpleConvert(account);
}
export function convertAnnouncement(announcement: Entity.Announcement) {
	return simpleConvert(announcement);
}
export function convertAttachment(attachment: Entity.Attachment) {
	return simpleConvert(attachment);
}
export function convertFilter(filter: Entity.Filter) {
	return simpleConvert(filter);
}
export function convertList(list: Entity.List) {
	return simpleConvert(list);
}
export function convertFeaturedTag(tag: Entity.FeaturedTag) {
	return simpleConvert(tag);
}

export function convertNotification(notification: Entity.Notification) {
	notification.account = convertAccount(notification.account);
	notification.id = toMastodonId(notification.id);
	if (notification.status)
		notification.status = convertStatus(notification.status);
	if (notification.reaction)
		notification.reaction = convertReaction(notification.reaction);
	return notification;
}

export function convertPoll(poll: Entity.Poll) {
	return simpleConvert(poll);
}
export function convertReaction(reaction: Entity.Reaction) {
	if (reaction.accounts) {
		reaction.accounts = reaction.accounts.map(convertAccount);
	}
	return reaction;
}
export function convertRelationship(relationship: Entity.Relationship) {
	return simpleConvert(relationship);
}

export function convertStatus(status: Entity.Status) {
	status.account = convertAccount(status.account);
	status.id = toMastodonId(status.id);
	if (status.in_reply_to_account_id)
		status.in_reply_to_account_id = toMastodonId(status.in_reply_to_account_id);
	if (status.in_reply_to_id)
		status.in_reply_to_id = toMastodonId(status.in_reply_to_id);
	status.media_attachments = status.media_attachments.map((attachment) =>
		convertAttachment(attachment),
	);
	status.mentions = status.mentions.map((mention) => ({
		...mention,
		id: toMastodonId(mention.id),
	}));
	if (status.poll) status.poll = convertPoll(status.poll);
	if (status.reblog) status.reblog = convertStatus(status.reblog);
	if (status.quote) status.quote = convertStatus(status.quote);
	status.reactions = status.reactions.map(convertReaction);

	return status;
}

export function convertConversation(conversation: Entity.Conversation) {
	conversation.id = toMastodonId(conversation.id);
	conversation.accounts = conversation.accounts.map(convertAccount);
	if (conversation.last_status) {
		conversation.last_status = convertStatus(conversation.last_status);
	}

	return conversation;
}
