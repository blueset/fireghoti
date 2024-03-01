import { Brackets } from "typeorm";
import { Notes } from "@/models/index.js";
import { Note } from "@/models/entities/note.js";
import define from "@/server/api/define.js";
import { makePaginationQuery } from "@/server/api/common/make-pagination-query.js";
import { generateVisibilityQuery } from "@/server/api/common/generate-visibility-query.js";
import { generateMutedUserQuery } from "@/server/api/common/generate-muted-user-query.js";
import { generateBlockedUserQuery } from "@/server/api/common/generate-block-query.js";
import { sqlLikeEscape } from "@/misc/sql-like-escape.js";

export const meta = {
	tags: ["notes"],

	requireCredential: false,
	requireCredentialPrivateMode: true,

	res: {
		type: "array",
		optional: false,
		nullable: false,
		items: {
			type: "object",
			optional: false,
			nullable: false,
			ref: "Note",
		},
	},

	errors: {},
} as const;

export const paramDef = {
	type: "object",
	properties: {
		query: { type: "string" },
		sinceId: { type: "string", format: "misskey:id" },
		untilId: { type: "string", format: "misskey:id" },
		sinceDate: { type: "number", nullable: true },
		untilDate: { type: "number", nullable: true },
		limit: { type: "integer", minimum: 1, maximum: 100, default: 10 },
		offset: { type: "integer", default: 0 },
		host: {
			type: "string",
			nullable: true,
			description: "The local host is represented with `null`.",
		},
		userId: {
			type: "string",
			format: "misskey:id",
			nullable: true,
			default: null,
		},
		withFiles: { type: "boolean", nullable: true },
		searchCwAndAlt: { type: "boolean", nullable: true },
		channelId: {
			type: "string",
			format: "misskey:id",
			nullable: true,
			default: null,
		},
		order: {
			type: "string",
			default: "chronological",
			nullable: true,
			description: "Either 'chronological' or 'relevancy'",
		},
	},
	required: ["query"],
} as const;

export default define(meta, paramDef, async (ps, me) => {
	const query = makePaginationQuery(
		Notes.createQueryBuilder("note"),
		ps.sinceId,
		ps.untilId,
		ps.sinceDate ?? undefined,
		ps.untilDate ?? undefined,
	);

	if (ps.userId != null) {
		query.andWhere("note.userId = :userId", { userId: ps.userId });
	}

	if (ps.channelId != null) {
		query.andWhere("note.channelId = :channelId", {
			channelId: ps.channelId,
		});
	}

	if (ps.query != null) {
		const q = sqlLikeEscape(ps.query);

		if (ps.searchCwAndAlt) {
			query.andWhere(
				new Brackets((qb) => {
					qb.where("note.text &@~ :q", { q })
						.orWhere("note.cw &@~ :q", { q })
						.orWhere(
							`EXISTS (
								SELECT FROM "drive_file"
								WHERE
									comment &@~ :q
								AND
									drive_file."id" = ANY(note."fileIds")
							)`,
							{ q },
						);
				}),
			);
		} else {
			query.andWhere("note.text &@~ :q", { q });
		}
	}

	query.innerJoinAndSelect("note.user", "user");

	// "from: me": search all (public, home, followers, specified) my posts
	//  otherwise: search public indexable posts only
	if (ps.userId == null || ps.userId !== me?.id) {
		query
			.andWhere("note.visibility = 'public'")
			.andWhere("user.isIndexable = TRUE");
	}

	if (ps.userId != null) {
		query.andWhere("note.userId = :userId", { userId: ps.userId });
	}

	if (ps.host === null) {
		query.andWhere("note.userHost IS NULL");
	}
	if (ps.host != null) {
		query.andWhere("note.userHost = :userHost", { userHost: ps.host });
	}

	if (ps.withFiles === true) {
		query.andWhere("note.fileIds != '{}'");
	}

	query
		.leftJoinAndSelect("user.avatar", "avatar")
		.leftJoinAndSelect("user.banner", "banner")
		.leftJoinAndSelect("note.reply", "reply")
		.leftJoinAndSelect("note.renote", "renote")
		.leftJoinAndSelect("reply.user", "replyUser")
		.leftJoinAndSelect("replyUser.avatar", "replyUserAvatar")
		.leftJoinAndSelect("replyUser.banner", "replyUserBanner")
		.leftJoinAndSelect("renote.user", "renoteUser")
		.leftJoinAndSelect("renoteUser.avatar", "renoteUserAvatar")
		.leftJoinAndSelect("renoteUser.banner", "renoteUserBanner");

	generateVisibilityQuery(query, me);
	if (me) generateMutedUserQuery(query, me);
	if (me) generateBlockedUserQuery(query, me);

	const notes: Note[] = await query.take(ps.limit).getMany();

	return await Notes.packMany(notes, me);
});
