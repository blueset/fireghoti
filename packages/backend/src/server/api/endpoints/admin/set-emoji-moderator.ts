import define from "@/server/api/define.js";
import { Users } from "@/models/index.js";
import { publishInternalEvent } from "@/services/stream.js";

export const meta = {
	tags: ["admin"],

	requireCredential: true,
	requireModerator: true,
} as const;

export const paramDef = {
	type: "object",
	properties: {
		userId: { type: "string", format: "misskey:id" },
		emojiModPerm: { type: "string" },
	},
	required: ["userId", "emojiModPerm"],
} as const;

export default define(meta, paramDef, async (ps) => {
	const user = await Users.findOneBy({ id: ps.userId });

	if (user == null) {
		throw new Error("user not found");
	}

	if (!["unauthorized", "add", "mod", "full"].includes(ps.emojiModPerm)) {
		throw new Error(
			"emojiModPerm must be 'unauthorized', 'add', 'mod', or 'full'",
		);
	}

	const _emojiModPerm =
		(ps.emojiModPerm as "unauthorized" | "add" | "mod" | "full") ??
		"unauthorized";

	await Users.update(user.id, {
		emojiModPerm: _emojiModPerm,
	});

	publishInternalEvent("userChangeModeratorState", {
		id: user.id,
		isModerator: true,
	});
});
