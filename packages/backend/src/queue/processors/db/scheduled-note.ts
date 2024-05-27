import { Users, Notes, DriveFiles } from "@/models/index.js";
import type { DbUserScheduledNoteData } from "@/queue/types.js";
import { queueLogger } from "../../logger.js";
import type Bull from "bull";
import deleteNote from "@/services/note/delete.js";
import createNote from "@/services/note/create.js";
import { In } from "typeorm";

const logger = queueLogger.createSubLogger("scheduled-post");

export async function scheduledNote(
	job: Bull.Job<DbUserScheduledNoteData>,
	done: () => void,
): Promise<void> {
	logger.info(`Creating: ${job.data.noteId}`);

	const user = await Users.findOneBy({ id: job.data.user.id });
	if (user == null) {
		done();
		return;
	}

	const draftNote = await Notes.findOneBy({ id: job.data.noteId });
	if (draftNote == null) {
		logger.warn(`Note ${job.data.noteId} does not exist`);
		done();
		return;
	}
	const files = await DriveFiles.findBy({ id: In(draftNote.fileIds) });

	if (user.isSuspended) {
		logger.info(`Cancelled due to user ${job.data.user.id} being suspended`);
		deleteNote(user, draftNote);
		done();
		return;
	}

	const visibleUsers = job.data.option.visibleUserIds
		? await Users.findBy({
				id: In(job.data.option.visibleUserIds),
			})
		: [];

	// Create scheduled (actual) note
	await createNote(user, {
		createdAt: new Date(),
		scheduledAt: null,
		files,
		poll: job.data.option.poll,
		text: draftNote.text || undefined,
		lang: draftNote.lang,
		reply: draftNote.reply,
		renote: draftNote.renote,
		cw: draftNote.cw,
		localOnly: draftNote.localOnly,
		visibility: job.data.option.visibility,
		visibleUsers,
		channel: draftNote.channel,
	});

	// Delete temporal (draft) note
	await deleteNote(user, draftNote);

	logger.info("Success");

	done();
}
