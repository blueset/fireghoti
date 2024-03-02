import { fetchMeta } from "@/misc/fetch-meta.js";
import { SwSubscriptions } from "@/models/index.js";
import define from "@/server/api/define.js";

export const meta = {
	tags: ["account"],

	requireCredential: true,

	description: "Check push notification registration exists.",

	res: {
		type: "object",
		optional: false,
		nullable: true,
		properties: {
			userId: {
				type: "string",
				optional: false,
				nullable: false,
			},
			endpoint: {
				type: "string",
				optional: false,
				nullable: false,
			},
			sendReadMessage: {
				type: "boolean",
				optional: false,
				nullable: false,
			},
			key: {
				type: "string",
				optional: false,
				nullable: true,
			},
		},
	},
} as const;

export const paramDef = {
	type: "object",
	properties: {
		endpoint: { type: "string", default: null },
	},
	required: [],
} as const;

// eslint-disable-next-line import/no-default-export
export default define(meta, paramDef, async (ps, me, token) => {

	const subscription = ps.endpoint ? await SwSubscriptions.findOneBy({
		userId: me.id,
		endpoint: ps.endpoint,
	}) : token ? await SwSubscriptions.findOneBy({
		userId: me.id,
		appAccessTokenId: token.id,
	}) : null;

	const instance = await fetchMeta(true);

	if (subscription != null) {
		return {
			userId: subscription.userId,
			endpoint: subscription.endpoint,
			sendReadMessage: subscription.sendReadMessage,
			key: instance.swPublicKey,
		};
	}

	return null;
});
