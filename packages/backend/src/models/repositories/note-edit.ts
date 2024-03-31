import { db } from "@/db/postgre.js";
import { NoteEdit } from "@/models/entities/note-edit.js";
import { awaitAll } from "@/prelude/await-all.js";
import type { Packed } from "@/misc/schema.js";
import { DriveFiles } from "../index.js";

export const NoteEditRepository = db.getRepository(NoteEdit).extend({
	async pack(noteEdit: NoteEdit) {
		const packed: Packed<"NoteEdit"> = await awaitAll({
			id: noteEdit.id,
			noteId: noteEdit.noteId,
			updatedAt: noteEdit.updatedAt.toISOString(),
			text: noteEdit.text,
			cw: noteEdit.cw,
			fileIds: noteEdit.fileIds,
			files: DriveFiles.packMany(noteEdit.fileIds),
		});

		return packed;
	},
	async packMany(noteEdits: NoteEdit[]) {
		if (noteEdits.length === 0) return [];

		const promises = await Promise.allSettled(
			noteEdits.map((n) => this.pack(n)),
		);

		// filter out rejected promises, only keep fulfilled values
		return promises.flatMap((result) =>
			result.status === "fulfilled" ? [result.value] : [],
		);
	},
});
