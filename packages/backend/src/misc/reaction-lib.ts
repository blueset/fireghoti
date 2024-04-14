import { fetchMeta, isUnicodeEmoji, toPuny } from "backend-rs";
import { Emojis } from "@/models/index.js";
import { IsNull } from "typeorm";

export function convertReactions(reactions: Record<string, number>) {
	const result = new Map();

	for (const reaction in reactions) {
		if (reactions[reaction] <= 0) continue;

		const decoded = decodeReaction(reaction).reaction;
		result.set(decoded, (result.get(decoded) || 0) + reactions[reaction]);
	}

	return Object.fromEntries(result);
}

export async function toDbReaction(
	reaction?: string | null,
	reacterHost?: string | null,
): Promise<string> {
	if (!reaction) return (await fetchMeta(true)).defaultReaction;

	if (reaction.includes("❤") || reaction.includes("♥️")) return "❤️";

	// Allow unicode reactions
	if (isUnicodeEmoji(reaction)) {
		return reaction;
	}

	reacterHost = reacterHost == null ? null : toPuny(reacterHost);

	const custom = reaction.match(/^:([\w+-]+)(?:@\.)?:$/);
	if (custom) {
		const name = custom[1];
		const emoji = await Emojis.findOneBy({
			host: reacterHost || IsNull(),
			name,
		});

		if (emoji) return reacterHost ? `:${name}@${reacterHost}:` : `:${name}:`;
	}

	return (await fetchMeta(true)).defaultReaction;
}

type DecodedReaction = {
	/**
	 * リアクション名 (Unicode Emoji or ':name@hostname' or ':name@.')
	 */
	reaction: string;

	/**
	 * name (カスタム絵文字の場合name, Emojiクエリに使う)
	 */
	name?: string;

	/**
	 * host (カスタム絵文字の場合host, Emojiクエリに使う)
	 */
	host?: string | null;
};

export function decodeReaction(str: string): DecodedReaction {
	const custom = str.match(/^:([\w+-]+)(?:@([\w.-]+))?:$/);

	if (custom) {
		const name = custom[1];
		const host = custom[2] || null;

		return {
			reaction: `:${name}@${host || "."}:`, // ローカル分は@以降を省略するのではなく.にする
			name,
			host,
		};
	}

	return {
		reaction: str,
		name: undefined,
		host: undefined,
	};
}
