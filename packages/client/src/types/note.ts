import type { noteVisibilities } from "firefish-js";

export type NoteVisibility = (typeof noteVisibilities)[number] | "private";
