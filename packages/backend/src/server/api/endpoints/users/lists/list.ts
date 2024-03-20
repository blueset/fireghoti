import { UserListJoinings, UserLists } from "@/models/index.js";
import define from "@/server/api/define.js";

export const meta = {
	tags: ["lists", "account"],

	requireCredential: true,

	kind: "read:account",

	description: "Show all lists that the authenticated user has created.",

	res: {
		type: "array",
		optional: false,
		nullable: false,
		items: {
			type: "object",
			optional: false,
			nullable: false,
			ref: "UserList",
		},
	},
} as const;

export const paramDef = {
	type: "object",
	properties: {
		userId: { type: "string", nullable: true },
	},
	required: [],
} as const;

export default define(meta, paramDef, async (ps, me) => {
	let query = UserLists.createQueryBuilder("userLists")
		.where("userLists.userId = :userId", { userId: me.id });
	if (ps.userId) {
        query = query.andWhere(
			`EXISTS(SELECT 1 FROM ${UserListJoinings.metadata.tableName} ulj WHERE ulj."userId" = :joinUserId AND ulj."userListId" = "userLists"."id")`, 
			{ joinUserId: ps.userId }
		);
    }
	const userLists = await query.getMany();

	return await Promise.all(userLists.map((x) => UserLists.pack(x)));
});
