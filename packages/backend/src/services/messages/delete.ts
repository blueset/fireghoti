import { config } from "@/config.js";
import { MessagingMessages, Users } from "@/models/index.js";
import type { MessagingMessage } from "@/models/entities/messaging-message.js";
import {
	publishToChatStream,
	publishToGroupChatStream,
	ChatEvent,
} from "backend-rs";
import { renderActivity } from "@/remote/activitypub/renderer/index.js";
import renderDelete from "@/remote/activitypub/renderer/delete.js";
import renderTombstone from "@/remote/activitypub/renderer/tombstone.js";
import { deliver } from "@/queue/index.js";

export async function deleteMessage(message: MessagingMessage) {
	await MessagingMessages.delete(message.id);
	postDeleteMessage(message);
}

async function postDeleteMessage(message: MessagingMessage) {
	if (message.recipientId) {
		const user = await Users.findOneByOrFail({ id: message.userId });
		const recipient = await Users.findOneByOrFail({ id: message.recipientId });

		if (Users.isLocalUser(user))
			publishToChatStream(
				message.userId,
				message.recipientId,
				ChatEvent.Deleted,
				message.id,
			);
		if (Users.isLocalUser(recipient))
			publishToChatStream(
				message.recipientId,
				message.userId,
				ChatEvent.Deleted,
				message.id,
			);

		if (Users.isLocalUser(user) && Users.isRemoteUser(recipient)) {
			const activity = renderActivity(
				renderDelete(
					renderTombstone(`${config.url}/notes/${message.id}`),
					user,
				),
			);
			deliver(user, activity, recipient.inbox);
		}
	} else if (message.groupId != null) {
		publishToGroupChatStream(message.groupId, ChatEvent.Deleted, message.id);
	}
}
