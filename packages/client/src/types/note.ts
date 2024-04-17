import type { noteVisibilities } from "firefish-js";

export type NoteVisibility = (typeof noteVisibilities)[number] | "private";

export interface NoteTranslation {
	sourceLang: string;
	text: string;
}
