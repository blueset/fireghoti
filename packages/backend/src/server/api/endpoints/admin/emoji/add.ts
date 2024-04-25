import define from "@/server/api/define.js";
import { Emojis, DriveFiles } from "@/models/index.js";
import { type ImageSize, genId, getImageSizeFromUrl } from "backend-rs";
import { insertModerationLog } from "@/services/insert-moderation-log.js";
import { ApiError } from "@/server/api/error.js";
import rndstr from "rndstr";
import { publishBroadcastStream } from "@/services/stream.js";
import { db } from "@/db/postgre.js";
import { apiLogger } from "@/server/api/logger.js";
import { inspect } from "node:util";

export const meta = {
	tags: ["admin", "emoji"],

	requireCredential: true,
	requireModerator: false,

	errors: {
		noSuchFile: {
			message: "No such file.",
			code: "MO_SUCH_FILE",
			id: "fc46b5a4-6b92-4c33-ac66-b806659bb5cf",
		},
		accessDenied: {
			message: "Access denied.",
			code: "ACCESS_DENIED",
			id: "fe8d7103-0ea8-4ec3-814d-f8b401dc69e9",
		},
	},
} as const;

export const paramDef = {
	type: "object",
	properties: {
		fileId: { type: "string", format: "misskey:id" },
	},
	required: ["fileId"],
} as const;

export default define(meta, paramDef, async (ps, me) => {
	// require emoji "add" permission
	if (!(me.isAdmin || me.isModerator || me.emojiModPerm !== "unauthorized"))
		throw new ApiError(meta.errors.accessDenied);

	const file = await DriveFiles.findOneBy({ id: ps.fileId });

	if (file == null) throw new ApiError(meta.errors.noSuchFile);

	const name = file.name.split(".")[0].match(/^[a-z0-9_]+$/)
		? file.name.split(".")[0]
		: `_${rndstr("a-z0-9", 8)}_`;

	let size: ImageSize | null = null;
	try {
		size = await getImageSizeFromUrl(file.url);
	} catch (err) {
		apiLogger.info(`Failed to determine the image size: ${file.url}`);
		apiLogger.debug(inspect(err));
	}

	const emoji = await Emojis.insert({
		id: genId(),
		updatedAt: new Date(),
		name: name,
		category: null,
		host: null,
		aliases: [],
		originalUrl: file.url,
		publicUrl: file.webpublicUrl ?? file.url,
		type: file.webpublicType ?? file.type,
		license: null,
		width: size?.width || null,
		height: size?.height || null,
	}).then((x) => Emojis.findOneByOrFail(x.identifiers[0]));

	await db.queryResultCache!.remove(["meta_emojis"]);

	publishBroadcastStream("emojiAdded", {
		emoji: await Emojis.pack(emoji.id),
	});

	insertModerationLog(me, "addEmoji", {
		emojiId: emoji.id,
	});

	return {
		id: emoji.id,
	};
});
