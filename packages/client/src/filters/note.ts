import type { entities } from "firefish-js";

export function notePage(
	note: entities.Note,
	options?: {
		historyPage?: boolean
}) {
	if (options?.historyPage) {
		return `/notes/${note.id}/history`;
	}
	return `/notes/${note.id}`;
};
