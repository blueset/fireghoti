import { SwSubscriptions } from "@/models/index.js";
import define from "@/server/api/define.js";

export const meta = {
	tags: ["account"],

	requireCredential: false,

	description: "Unregister from receiving push notifications.",
} as const;

export const paramDef = {
	type: "object",
	properties: {
		endpoint: { type: "string", default: null },
	},
	required: [],
} as const;

export default define(meta, paramDef, async (ps, me, token) => {
	if (ps.endpoint) {
		await SwSubscriptions.delete({
			...(me ? { userId: me.id } : {}),
			endpoint: ps.endpoint,
		});
	} else if (token) {
		await SwSubscriptions.delete({
			...(me ? { userId: me.id } : {}),
			appAccessTokenId: token.id,
		});
	}
});
