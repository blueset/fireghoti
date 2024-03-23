import RE2 from "re2";
import type { Note } from "@/models/entities/note.js";

type NoteLike = {
	userId: Note["userId"];
	text: Note["text"];
	files?: Note["files"];
	cw?: Note["cw"];
	reply?: NoteLike | null;
	renote?: NoteLike | null;
};

function checkWordMute(
	note: NoteLike | null | undefined,
	mutedWords: string[][],
	mutedPatterns: string[],
): boolean {
	if (note == null) return false;

	let text = `${note.cw ?? ""} ${note.text ?? ""}`;
	if (note.files != null)
		text += ` ${note.files.map((f) => f.comment ?? "").join(" ")}`;
	text = text.trim();

	if (text === "") return false;

	for (const mutedWord of mutedWords) {
		// Clean up
		const keywords = mutedWord.filter((keyword) => keyword !== "");

		if (
			keywords.length > 0 &&
			keywords.every((keyword) =>
				text.toLowerCase().includes(keyword.toLowerCase()),
			)
		)
			return true;
	}

	for (const mutedPattern of mutedPatterns) {
		// represents RegExp
		const regexp = mutedPattern.match(/^\/(.+)\/(.*)$/);

		// This should never happen due to input sanitisation.
		if (!regexp) {
			console.warn(`Found invalid regex in word mutes: ${mutedPattern}`);
			continue;
		}

		try {
			if (new RE2(regexp[1], regexp[2]).test(text)) return true;
		} catch (err) {
			// This should never happen due to input sanitisation.
		}
	}

	return false;
}

export async function getWordHardMute(
	note: NoteLike | null,
	mutedWords: string[][],
	mutedPatterns: string[],
): Promise<boolean> {
	if (note == null || mutedWords == null || mutedPatterns == null) return false;

	if (mutedWords.length > 0) {
		return (
			checkWordMute(note, mutedWords, mutedPatterns) ||
			checkWordMute(note.reply, mutedWords, mutedPatterns) ||
			checkWordMute(note.renote, mutedWords, mutedPatterns)
		);
	}

	return false;
}
