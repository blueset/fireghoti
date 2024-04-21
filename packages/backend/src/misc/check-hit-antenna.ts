import type { Antenna } from "@/models/entities/antenna.js";
import type { Note } from "@/models/entities/note.js";
import type { User } from "@/models/entities/user.js";
import type { UserProfile } from "@/models/entities/user-profile.js";
import { Blockings, Followings, UserProfiles } from "@/models/index.js";
import { checkWordMute, getFullApAccount, stringToAcct } from "backend-rs";
import type { Packed } from "@/misc/schema.js";
import { Cache } from "@/misc/cache.js";

const blockingCache = new Cache<User["id"][]>("blocking", 60 * 5);
const hardMutesCache = new Cache<{
	userId: UserProfile["userId"];
	mutedWords: UserProfile["mutedWords"];
	mutedPatterns: UserProfile["mutedPatterns"];
}>("hardMutes", 60 * 5);
const followingCache = new Cache<User["id"][]>("following", 60 * 5);

export async function checkHitAntenna(
	antenna: Antenna,
	note: Note | Packed<"Note">,
	noteUser: { id: User["id"]; username: string; host: string | null },
): Promise<boolean> {
	if (note.visibility === "specified") return false;
	if (antenna.withFile) {
		if (note.fileIds && note.fileIds.length === 0) return false;
	}
	if (!antenna.withReplies && note.replyId != null) return false;

	if (antenna.src === "users") {
		const accts = antenna.users.map((x) => {
			const { username, host } = stringToAcct(x);
			return getFullApAccount(username, host).toLowerCase();
		});
		if (
			!accts.includes(
				getFullApAccount(noteUser.username, noteUser.host).toLowerCase(),
			)
		)
			return false;
	} else if (antenna.src === "instances") {
		const instances = antenna.instances
			.filter((x) => x !== "")
			.map((host) => {
				return host.toLowerCase();
			});
		if (!instances.includes(noteUser.host?.toLowerCase() ?? "")) return false;
	}

	const keywords = antenna.keywords
		// Clean up
		.map((xs) => xs.filter((x) => x !== ""))
		.filter((xs) => xs.length > 0);

	let text = `${note.text ?? ""} ${note.cw ?? ""}`;
	if (note.files != null)
		text += ` ${note.files.map((f) => f.comment ?? "").join(" ")}`;
	text = text.trim();

	if (keywords.length > 0) {
		if (note.text == null) return false;

		const matched = keywords.some((and) =>
			and.every((keyword) =>
				antenna.caseSensitive
					? text.includes(keyword)
					: text.toLowerCase().includes(keyword.toLowerCase()),
			),
		);

		if (!matched) return false;
	}

	const excludeKeywords = antenna.excludeKeywords
		// Clean up
		.map((xs) => xs.filter((x) => x !== ""))
		.filter((xs) => xs.length > 0);

	if (excludeKeywords.length > 0) {
		if (note.text == null) return false;

		const matched = excludeKeywords.some((and) =>
			and.every((keyword) =>
				antenna.caseSensitive
					? note.text?.includes(keyword)
					: note.text?.toLowerCase().includes(keyword.toLowerCase()),
			),
		);

		if (matched) return false;
	}

	// アンテナ作成者がノート作成者にブロックされていたらスキップ
	const blockings = await blockingCache.fetch(noteUser.id, () =>
		Blockings.findBy({ blockerId: noteUser.id }).then((res) =>
			res.map((x) => x.blockeeId),
		),
	);
	if (blockings.includes(antenna.userId)) return false;

	if (note.visibility === "followers" || note.visibility === "home") {
		const following = await followingCache.fetch(antenna.userId, () =>
			Followings.find({
				where: { followerId: antenna.userId },
				select: ["followeeId"],
			}).then((relations) => relations.map((relation) => relation.followeeId)),
		);
		if (!following.includes(note.userId)) return false;
	}

	const mutes = await hardMutesCache.fetch(antenna.userId, () =>
		UserProfiles.findOneByOrFail({
			userId: antenna.userId,
		}).then((profile) => {
			return {
				userId: antenna.userId,
				mutedWords: profile.mutedWords,
				mutedPatterns: profile.mutedPatterns,
			};
		}),
	);
	if (
		mutes.mutedWords != null &&
		mutes.mutedPatterns != null &&
		antenna.userId !== note.userId &&
		(await checkWordMute(note, mutes.mutedWords, mutes.mutedPatterns))
	)
		return false;

	// TODO: eval expression

	return true;
}
