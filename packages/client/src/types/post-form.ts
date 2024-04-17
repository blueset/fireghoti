import type { entities } from "firefish-js";

export type PollType = {
	choices: string[];
	multiple: boolean;
	expiresAt: string | null;
	expiredAfter: number | null;
};

export type NoteDraft = entities.Note & {
	poll?: PollType;
};
