import { fetchMeta } from "backend-rs";
import { genId } from "backend-rs";
import { SwSubscriptions } from "@/models/index.js";
import define from "@/server/api/define.js";

export const meta = {
	tags: ["account"],

	requireCredential: true,

	description: "Register to receive push notifications.",

	res: {
		type: "object",
		optional: false,
		nullable: false,
		properties: {
			state: {
				type: "string",
				optional: true,
				nullable: false,
				enum: ["already-subscribed", "subscribed"],
			},
			key: {
				type: "string",
				optional: false,
				nullable: true,
			},
			userId: {
				type: "string",
				optional: true,
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
		},
	},
} as const;

export const paramDef = {
	type: "object",
	properties: {
		endpoint: { type: "string" },
		auth: { type: "string" },
		publickey: { type: "string" },
		sendReadMessage: { type: "boolean", default: false },
		isMastodon: { type: "boolean", default: false },
	},
	required: ["endpoint", "auth", "publickey"],
} as const;

export default define(meta, paramDef, async (ps, me, token) => {
	const subscription = await SwSubscriptions.findOneBy({
		userId: me.id,
		endpoint: ps.endpoint,
		auth: ps.auth,
		publickey: ps.publickey,
	});

	const instance = await fetchMeta(false);

	// if already subscribed
	if (subscription != null) {
		return {
			state: "already-subscribed" as const,
			key: instance.swPublicKey,
			userId: me.id,
			endpoint: subscription.endpoint,
			sendReadMessage: subscription.sendReadMessage,
		};
	}

	if (ps.isMastodon && token == null) {
		throw new Error("Mastodon app access token is required");
	}

	if (ps.isMastodon && token) {
		await SwSubscriptions.delete({
			userId: me.id,
			appAccessTokenId: token.id,
		});
	}

	await SwSubscriptions.insert({
		id: genId(),
		createdAt: new Date(),
		userId: me.id,
		endpoint: ps.endpoint,
		auth: ps.auth,
		publickey: ps.publickey,
		sendReadMessage: ps.sendReadMessage,
		appAccessTokenId: ps.isMastodon && token ? token.id : undefined,
	});

	return {
		state: "subscribed" as const,
		key: instance.swPublicKey,
		userId: me.id,
		endpoint: ps.endpoint,
		sendReadMessage: ps.sendReadMessage,
	};
});
