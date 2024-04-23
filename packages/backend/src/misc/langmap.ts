import LangMap from "firefish-js/built/misc/langmap.js";

export const langmap = LangMap.langmap;

export type PostLanguage = keyof typeof langmap;
