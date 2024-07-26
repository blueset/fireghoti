import { config } from "@/config.js";
import type { Relay } from "@/models/entities/relay.js";
import type { ILocalUser } from "@/models/entities/user.js";

export function renderFollowRelay(relay: Relay, relayActorId: string) {
	const follow = {
		id: `${config.url}/activities/follow-relay/${relay.id}`,
		type: "Follow",
		actor: `${config.url}/users/${relayActorId}`,
		object: "https://www.w3.org/ns/activitystreams#Public",
	};

	return follow;
}
