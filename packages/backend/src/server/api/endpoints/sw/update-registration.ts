import { fetchMeta } from "backend-rs";
import { SwSubscriptions } from "@/models/index.js";
import define from "@/server/api/define.js";

export const meta = {
	tags: ["account"],

	requireCredential: true,

	description: "Unregister from receiving push notifications.",
} as const;

export const paramDef = {
	type: "object",
	properties: {
		endpoint: { type: "string", default: null },
		sendReadMessage: { type: "boolean" },
	},
	required: [],
} as const;

export default define(meta, paramDef, async (ps, me, token) => {
	const swSubscription = ps.endpoint
		? await SwSubscriptions.findOneBy({
				userId: me.id,
				endpoint: ps.endpoint,
			})
		: token
			? await SwSubscriptions.findOneBy({
					userId: me.id,
					appAccessTokenId: token.id,
				})
			: null;

	if (swSubscription === null) {
		throw new Error("No such registration");
	}

	if (ps.sendReadMessage !== undefined) {
		swSubscription.sendReadMessage = ps.sendReadMessage;
	}

	await SwSubscriptions.update(swSubscription.id, {
		sendReadMessage: swSubscription.sendReadMessage,
	});

	const instance = await fetchMeta(true);

	return {
		userId: swSubscription.userId,
		endpoint: swSubscription.endpoint,
		sendReadMessage: swSubscription.sendReadMessage,
		key: instance.swPublicKey,
	};
});
