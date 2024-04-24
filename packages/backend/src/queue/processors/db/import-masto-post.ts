import create from "@/services/note/create.js";
import { Users } from "@/models/index.js";
import type { DbUserImportMastoPostJobData } from "@/queue/types.js";
import { queueLogger } from "../../logger.js";
import type Bull from "bull";
import { htmlToMfm } from "@/remote/activitypub/misc/html-to-mfm.js";
import { resolveNote } from "@/remote/activitypub/models/note.js";
import { uploadFromUrl } from "@/services/drive/upload-from-url.js";
import type { DriveFile } from "@/models/entities/drive-file.js";
import { Notes, NoteEdits } from "@/models/index.js";
import type { Note } from "@/models/entities/note.js";
import { genId } from "backend-rs";

const logger = queueLogger.createSubLogger("import-masto-post");

export async function importMastoPost(
	job: Bull.Job<DbUserImportMastoPostJobData>,
	done: any,
): Promise<void> {
	const user = await Users.findOneBy({ id: job.data.user.id });
	if (user == null) {
		done();
		return;
	}
	const post = job.data.post;
	const isRenote = post.type === "Announce";
	let reply: Note | null = null;
	let renote: Note | null = null;
	job.progress(20);
	if (!isRenote && post.object.inReplyTo != null) {
		reply = await resolveNote(post.object.inReplyTo);
	}
	// renote also need resolve original note
	if (isRenote) {
		renote = await resolveNote(post.object);
	}
	job.progress(40);
	if (post.directMessage) {
		done();
		return;
	}
	if (job.data.signatureCheck) {
		if (!post.signature) {
			done();
			return;
		}
	}
	job.progress(60);
	let text;
	try {
		text = isRenote
			? undefined
			: htmlToMfm(post.object.content, post.object.tag);
	} catch (e) {
		throw e;
	}
	job.progress(80);

	let files: DriveFile[] = (post.object?.attachment || [])
		.map((x: any) => x?.driveFile)
		.filter((x: any) => x);

	if (!isRenote && files.length == 0) {
		const urls = post.object.attachment
			.map((x: any) => x.url)
			.filter((x: String) => x.startsWith("http"));
		files = [];
		for (const url of urls) {
			try {
				const file = await uploadFromUrl({
					url: url,
					user: user,
				});
				files.push(file);
			} catch (e) {
				logger.warn(`Skipped adding file to drive: ${url}`);
			}
		}
	}
	let note = await Notes.findOneBy({
		createdAt: isRenote
			? new Date(post.published)
			: new Date(post.object.published),
		text: text,
		userId: user.id,
	});

	if (note && (note?.fileIds?.length || 0) < files.length) {
		const update: Partial<Note> = {};
		update.fileIds = files.map((x) => x.id);
		await Notes.update(note.id, update);
		await NoteEdits.insert({
			id: genId(),
			noteId: note.id,
			text: note.text || undefined,
			cw: note.cw,
			fileIds: note.fileIds,
			updatedAt: new Date(),
		});
		logger.info("Post updated");
	}
	if (!note) {
		note = await create(user, {
			createdAt: isRenote
				? new Date(post.published)
				: new Date(post.object.published),
			files: files.length === 0 ? undefined : files,
			poll: undefined,
			text: text || undefined,
			reply,
			renote,
			cw: !isRenote && post.object.sensitive ? post.object.summary : undefined,
			localOnly: false,
			visibility: "hiddenpublic",
			visibleUsers: [],
			channel: null,
			apMentions: new Array(0),
			apHashtags: undefined,
			apEmojis: undefined,
		});
		logger.debug("New post has been created");
	} else {
		logger.info("This post already exists");
	}
	job.progress(100);
	done();

	logger.info("Imported");
}
