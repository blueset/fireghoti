import { config } from "@/config.js";
import { Notes, Users } from "@/models/index.js";
import { deliver } from "@/queue/index.js";
import { renderActivity } from "@/remote/activitypub/renderer/index.js";
import type { CacheableRemoteUser } from "@/models/entities/user.js";
import type { IQuoteRequest } from "../type.js";
import { getApId } from "../type.js";
import { apLogger } from "../logger.js";
import { inspect } from "node:util";

/**
 * FEP-044f: Handle incoming QuoteRequest activities.
 *
 * Since we use a hard-coded permissive quoting policy (all public/home posts
 * allow quoting by anyone), we always respond with Accept and a stateless
 * QuoteAuthorization stamp URI.
 */
export default async function quoteRequest(
	actor: CacheableRemoteUser,
	activity: IQuoteRequest,
): Promise<string> {
	const uri = activity.id || activity;
	apLogger.info(`QuoteRequest: ${uri}`);

	// The object is the URI of the local note being quoted
	const quotedObjectUri =
		typeof activity.object === "string"
			? activity.object
			: activity.object?.id;

	if (!quotedObjectUri) {
		return "skip: QuoteRequest missing object";
	}

	// The instrument is the URI of the remote quote post
	const instrumentUri =
		typeof activity.instrument === "string"
			? activity.instrument
			: activity.instrument?.id;

	if (!instrumentUri) {
		return "skip: QuoteRequest missing instrument";
	}

	// Resolve the quoted local note
	// The object URI should be like ${config.url}/notes/${noteId}
	const noteIdMatch = quotedObjectUri.match(
		new RegExp(`^${escapeRegex(config.url)}/notes/([a-zA-Z0-9]+)$`),
	);

	let note;
	if (noteIdMatch) {
		note = await Notes.findOneBy({ id: noteIdMatch[1] });
	} else {
		// Try by URI as fallback
		note = await Notes.findOneBy({ uri: quotedObjectUri });
	}

	if (note == null) {
		apLogger.info(
			`QuoteRequest: quoted note not found: ${quotedObjectUri}`,
		);
		return "skip: QuoteRequest quoted note not found";
	}

	// Must be a local note
	if (note.userHost != null) {
		apLogger.info("QuoteRequest: quoted note is not local");
		return "skip: QuoteRequest quoted note is not local";
	}

	// Only allow quoting public/home notes
	if (!["public", "home"].includes(note.visibility)) {
		apLogger.info(
			`QuoteRequest: quoted note visibility is ${note.visibility}, rejecting`,
		);

		// Send a Reject
		const localUser = await Users.findOneBy({ id: note.userId });
		if (localUser && actor.inbox) {
			const rejectActivity = renderActivity({
				type: "Reject",
				actor: `${config.url}/users/${localUser.id}`,
				to: actor.uri,
				object: {
					type: "QuoteRequest",
					id: typeof uri === "string" ? uri : undefined,
					actor: actor.uri,
					object: quotedObjectUri,
					instrument: instrumentUri,
				},
			});
			if (rejectActivity) {
				deliver(localUser.id, rejectActivity, actor.inbox);
			}
		}
		return "reject: QuoteRequest for non-public note";
	}

	const localUser = await Users.findOneBy({ id: note.userId });
	if (localUser == null) {
		return "skip: QuoteRequest local note author not found";
	}

	// Compute the deterministic stateless stamp URI
	const encodedQuoteUri = Buffer.from(instrumentUri, "utf-8").toString(
		"base64url",
	);
	const stampUri = `${config.url}/quote-authorizations/${note.id}/${encodedQuoteUri}`;

	// Build and send Accept with QuoteAuthorization result
	const acceptActivity = renderActivity({
		type: "Accept",
		actor: `${config.url}/users/${localUser.id}`,
		to: actor.uri,
		object: {
			type: "QuoteRequest",
			id: typeof uri === "string" ? uri : undefined,
			actor: actor.uri,
			object: quotedObjectUri,
			instrument: instrumentUri,
		},
		result: stampUri,
	});

	if (acceptActivity && actor.inbox) {
		deliver(localUser.id, acceptActivity, actor.inbox);
		apLogger.info(
			`QuoteRequest: accepted quote of ${quotedObjectUri} by ${actor.uri}, stamp: ${stampUri}`,
		);
	}

	return "ok";
}

function escapeRegex(str: string): string {
	return str.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
}
