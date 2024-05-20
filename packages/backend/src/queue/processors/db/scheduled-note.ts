import { Users, Notes, ScheduledNotes } from "@/models/index.js";
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

	const note = await Notes.findOneBy({ id: job.data.noteId });
	if (note == null) {
		done();
		return;
	}

	if (user.isSuspended) {
		deleteNote(user, note);
		done();
		return;
	}

	await ScheduledNotes.delete({
		noteId: note.id,
		userId: user.id,
	});

	const visibleUsers = job.data.option.visibleUserIds
		? await Users.findBy({
				id: In(job.data.option.visibleUserIds),
			})
		: [];

	await createNote(user, {
		createdAt: new Date(),
		files: note.files,
		poll: job.data.option.poll,
		text: note.text || undefined,
		lang: note.lang,
		reply: note.reply,
		renote: note.renote,
		cw: note.cw,
		localOnly: note.localOnly,
		visibility: job.data.option.visibility,
		visibleUsers,
		channel: note.channel,
	});

	await deleteNote(user, note);

	logger.info("Success");

	done();
}
