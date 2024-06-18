import { Notes } from "@/models/index.js";
import type { Note } from "@/models/entities/note.js";
import define from "@/server/api/define.js";
import { makePaginationQuery } from "@/server/api/common/make-pagination-query.js";
import { generateVisibilityQuery } from "@/server/api/common/generate-visibility-query.js";
import { generateMutedUserQuery } from "@/server/api/common/generate-muted-user-query.js";
import { generateBlockedUserQuery } from "@/server/api/common/generate-block-query.js";
import { sqlLikeEscape } from "backend-rs";
import type { SelectQueryBuilder } from "typeorm";

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
	async function search(
		modifier?: (query: SelectQueryBuilder<Note>) => void,
	): Promise<Note[]> {
		const query = makePaginationQuery(
			Notes.createQueryBuilder("note"),
			ps.sinceId,
			ps.untilId,
			ps.sinceDate ?? undefined,
			ps.untilDate ?? undefined,
		);
		modifier?.(query);

		if (ps.userId != null) {
			query.andWhere("note.userId = :userId", { userId: ps.userId });
		}

		if (ps.channelId != null) {
			query.andWhere("note.channelId = :channelId", {
				channelId: ps.channelId,
			});
		}

		query.innerJoinAndSelect("note.user", "user");

		// "from: me": search all (public, home, followers, specified) my posts
		//  otherwise: search public indexable posts only
		// if (ps.userId == null || ps.userId !== me?.id) {
		// 	query
		// 		.andWhere("note.visibility = 'public'")
		// 		.andWhere("user.isIndexable = TRUE");
		// }

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

		return await query.take(ps.limit).getMany();
	}

	let notes: Note[];

	if (ps.query != null) {
		const q = sqlLikeEscape(ps.query);

		if (ps.searchCwAndAlt) {
			// Whether we should return latest notes first
			const isDescendingOrder =
				(ps.sinceId == null || ps.untilId != null) &&
				(ps.sinceId != null ||
					ps.untilId != null ||
					ps.sinceDate == null ||
					ps.untilDate != null);

			const compare = isDescendingOrder
				? (lhs: Note, rhs: Note) =>
						Math.sign(rhs.createdAt.getTime() - lhs.createdAt.getTime())
				: (lhs: Note, rhs: Note) =>
						Math.sign(lhs.createdAt.getTime() - rhs.createdAt.getTime());

			notes = [
				...new Map(
					(
						await Promise.all([
							search((query) => {
								query.andWhere("note.text &@~ :q", { q });
							}),
							search((query) => {
								query.andWhere("note.cw &@~ :q", { q });
							}),
							search((query) => {
								query
									.andWhere("drive_file.comment &@~ :q", { q })
									.innerJoin("note.files", "drive_file");
							}),
						])
					)
						.flatMap((e) => e)
						.map((note) => [note.id, note]),
				).values(),
			]
				.sort(compare)
				.slice(0, ps.limit);
		} else {
			notes = await search((query) => {
				query.andWhere("note.text &@~ :q", { q });
			});
		}
	} else {
		notes = await search();
	}

	return await Notes.packMany(notes, me);
});
