import { ILocalUser, User } from "@/models/entities/user.js";
import { getNote } from "@/server/api/common/getters.js";
import { Note } from "@/models/entities/note.js";
import config from "@/config/index.js";
import mfm, { MfmLink, MfmUrl } from "mfm-js";
import { UserConverter } from "@/server/api/mastodon/converters/user.js";
import { VisibilityConverter } from "@/server/api/mastodon/converters/visibility.js";
import { escapeMFM } from "@/server/api/mastodon/converters/mfm.js";
import {
	aggregateNoteEmojis,
	PopulatedEmoji,
	populateEmojis,
	prefetchEmojis,
} from "@/misc/populate-emojis.js";
import { EmojiConverter } from "@/server/api/mastodon/converters/emoji.js";
import {
	DriveFiles,
	NoteFavorites,
	NoteReactions,
	Notes,
	NoteThreadMutings,
	UserNotePinings,
} from "@/models/index.js";
import { decodeReaction } from "@/misc/reaction-lib.js";
import { MentionConverter } from "@/server/api/mastodon/converters/mention.js";
import { PollConverter } from "@/server/api/mastodon/converters/poll.js";
import { populatePoll } from "@/models/repositories/note.js";
import { FileConverter } from "@/server/api/mastodon/converters/file.js";
import { awaitAll } from "@/prelude/await-all.js";
import {
	AccountCache,
	UserHelpers,
} from "@/server/api/mastodon/helpers/user.js";
import { In, IsNull } from "typeorm";
import { MfmHelpers } from "@/server/api/mastodon/helpers/mfm.js";
import {
	getStubMastoContext,
	MastoContext,
} from "@/server/api/mastodon/index.js";
import { NoteHelpers } from "@/server/api/mastodon/helpers/note.js";
import isQuote from "@/misc/is-quote.js";
import { unique } from "@/prelude/array.js";
import { NoteReaction } from "@/models/entities/note-reaction.js";
import { Cache } from "@/misc/cache.js";
import { isFiltered } from "@/misc/is-filtered.js";
import { unfurl } from "unfurl.js";

export class NoteConverter {
	private static noteContentHtmlCache = new Cache<string | null>(
		"html:note:content",
		config.htmlCache?.ttlSeconds ?? 60 * 60,
	);
	private static cardCache = new Cache<MastodonEntity.Card | null>(
		"note:card",
		60 * 60,
	);
	public static async encode(
		note: Note,
		ctx: MastoContext,
		recurseCounter: number = 2,
	): Promise<MastodonEntity.Status> {
		const user = ctx.user as ILocalUser | null;
		const noteUser = note.user ?? UserHelpers.getUserCached(note.userId, ctx);

		if (!(await Notes.isVisibleForMe(note, user?.id ?? null)))
			throw new Error("Cannot encode note not visible for user");

		const host = Promise.resolve(noteUser).then(
			(noteUser) => noteUser.host ?? null,
		);

		const reactionEmojiNames = Object.keys(note.reactions)
			.filter((x) => x?.startsWith(":"))
			.map((x) => decodeReaction(x).reaction)
			.map((x) => x.replace(/:/g, ""));

		const populated = host.then(async (host) =>
			populateEmojis(note.emojis.concat(reactionEmojiNames), host),
		);

		const noteEmoji = populated.then((noteEmoji) =>
			noteEmoji
				.filter((e) => e.name.indexOf("@") === -1)
				.map((e) => EmojiConverter.encode(e)),
		);

		const reactionCount = Object.values(note.reactions).reduce(
			(a, b) => a + b,
			0,
		);

		const aggregateReaction = (
			ctx.reactionAggregate as Map<string, NoteReaction | null>
		)?.get(note.id);

		const reaction =
			aggregateReaction !== undefined
				? aggregateReaction
				: user
				  ? NoteReactions.findOneBy({
							userId: user.id,
							noteId: note.id,
					  })
				  : null;

		const isFavorited = Promise.resolve(reaction).then((p) => !!p);

		const isReblogged =
			(ctx.renoteAggregate as Map<string, boolean>)?.get(note.id) ??
			(user
				? Notes.exist({
						where: {
							userId: user.id,
							renoteId: note.id,
							text: IsNull(),
						},
				  })
				: null);

		const renote =
			note.renote ??
			(note.renoteId && recurseCounter > 0
				? getNote(note.renoteId, user)
				: null);

		const isBookmarked =
			(ctx.bookmarkAggregate as Map<string, boolean>)?.get(note.id) ??
			(user
				? NoteFavorites.exist({
						where: {
							userId: user.id,
							noteId: note.id,
						},
						take: 1,
				  })
				: false);

		const isMuted =
			(ctx.mutingAggregate as Map<string, boolean>)?.get(
				note.threadId ?? note.id,
			) ??
			(user
				? NoteThreadMutings.exist({
						where: {
							userId: user.id,
							threadId: note.threadId || note.id,
						},
				  })
				: false);

		const files = DriveFiles.packMany(note.fileIds);

		const mentions = Promise.all(
			note.mentions.map((p) =>
				UserHelpers.getUserCached(p, ctx)
					.then((u) =>
						MentionConverter.encode(u, JSON.parse(note.mentionedRemoteUsers)),
					)
					.catch(() => null),
			),
		).then((p) => p.filter((m) => m)) as Promise<MastodonEntity.Mention[]>;

		const quoteUri = Promise.resolve(renote).then((renote) => {
			if (!renote || !isQuote(note)) return null;
			return renote.url ?? renote.uri ?? `${config.url}/notes/${renote.id}`;
		});

		const identifier = `${note.id}:${(
			note.updatedAt ?? note.createdAt
		).getTime()}`;

		const text = quoteUri.then((quoteUri) =>
			note.text !== null
				? quoteUri !== null
					? note.text
							.replaceAll(`RE: ${quoteUri}`, "")
							.replaceAll(quoteUri, "")
							.trimEnd()
					: note.text
				: null,
		);

		const content = this.noteContentHtmlCache
			.fetch(
				identifier,
				async () =>
					text.then((text) =>
						text !== null
							? quoteUri
									.then((quoteUri) =>
										MfmHelpers.toHtml(
											mfm.parse(text),
											JSON.parse(note.mentionedRemoteUsers),
											note.userHost,
											false,
											quoteUri,
										),
									)
									.then((p) => p ?? escapeMFM(text))
							: "",
					),
				true,
			)
			.then((p) => p ?? "");

		const card = text
			.then(async (text) => this.extractUrlFromMfm(text))
			.then(async (urls) =>
				!urls
					? null
					: Promise.race([
							this.cardCache.fetch(
								identifier,
								async () => this.generateCard(urls, note.lang ?? undefined),
								true,
							),
							new Promise<null>((resolve) =>
								setTimeout(() => resolve(null), 5000),
							), // Timeout card generation after 5 seconds
					  ]),
			);

		const isPinned =
			(ctx.pinAggregate as Map<string, boolean>)?.get(note.id) ??
			(user && note.userId === user.id
				? UserNotePinings.exist({ where: { userId: user.id, noteId: note.id } })
				: undefined);

		const tags = note.tags.map((tag) => {
			return {
				name: tag,
				url: `${config.url}/tags/${tag}`,
			} as MastodonEntity.Tag;
		});

		const reblog = Promise.resolve(renote).then((renote) =>
			recurseCounter > 0 && renote
				? this.encode(
						renote,
						ctx,
						isQuote(renote) && !isQuote(note) ? --recurseCounter : 0,
				  )
				: null,
		);

		const filtered = isFiltered(note, user).then((res) => {
			if (
				!res ||
				ctx.filterContext == null ||
				!["home", "public"].includes(ctx.filterContext)
			)
				return null;
			return [
				{
					filter: {
						id: "0",
						title: "Hard word mutes",
						context: ["home", "public"],
						expires_at: null,
						filter_action: "hide",
						keywords: [],
						statuses: [],
					},
				} as MastodonEntity.FilterResult,
			];
		});

		// noinspection ES6MissingAwait
		return await awaitAll({
			id: note.id,
			uri: note.uri ?? `https://${config.host}/notes/${note.id}`,
			url: note.url ?? note.uri ?? `https://${config.host}/notes/${note.id}`,
			account: Promise.resolve(noteUser).then((p) =>
				UserConverter.encode(p, ctx),
			),
			in_reply_to_id: note.replyId,
			in_reply_to_account_id: note.replyUserId,
			reblog: reblog.then((reblog) => (!isQuote(note) ? reblog : null)),
			content: content,
			content_type: "text/x.misskeymarkdown",
			text: text,
			created_at: note.createdAt.toISOString(),
			emojis: noteEmoji,
			replies_count: reblog.then(
				(reblog) =>
					(!isQuote(note) ? reblog?.replies_count : note.repliesCount) ?? 0,
			),
			reblogs_count: reblog.then(
				(reblog) =>
					(!isQuote(note) ? reblog?.reblogs_count : note.renoteCount) ?? 0,
			),
			favourites_count: reactionCount,
			reblogged: reblog.then((reblog) =>
				reblog && !isQuote(note) ? true : false,
			),
			favourited: isFavorited,
			muted: isMuted,
			sensitive: files.then((files) =>
				files.length > 0 ? files.some((f) => f.isSensitive) : false,
			),
			spoiler_text: note.cw ? note.cw : "",
			visibility: VisibilityConverter.encode(note.visibility),
			media_attachments: files.then((files) =>
				files.length > 0 ? files.map((f) => FileConverter.encode(f)) : [],
			),
			mentions: mentions,
			tags: tags,
			card: card,
			poll: note.hasPoll
				? populatePoll(note, user?.id ?? null).then((p) =>
						noteEmoji.then((emojis) =>
							PollConverter.encode(p, note.id, emojis),
						),
				  )
				: null,
			application: null, //FIXME
			language: note.lang,
			pinned: isPinned,
			reactions: populated.then((populated) =>
				Promise.resolve(reaction).then((reaction) =>
					this.encodeReactions(
						note.reactions,
						reaction?.reaction,
						populated,
						ctx,
					),
				),
			),
			bookmarked: isBookmarked,
			quote: reblog.then((reblog) => (isQuote(note) ? reblog : null)),
			edited_at: note.updatedAt?.toISOString() ?? null,
			filtered: filtered,
		});
	}

	public static async encodeMany(
		notes: Note[],
		ctx: MastoContext,
	): Promise<MastodonEntity.Status[]> {
		await this.aggregateData(notes, ctx);
		const encoded = notes.map((n) => this.encode(n, ctx));
		return Promise.all(encoded);
	}

	public static async aggregateData(
		notes: Note[],
		ctx: MastoContext,
	): Promise<void> {
		if (notes.length === 0) return;

		const user = ctx.user as ILocalUser | null;
		const reactionAggregate = new Map<Note["id"], NoteReaction | null>();
		const renoteAggregate = new Map<Note["id"], boolean>();
		const mutingAggregate = new Map<Note["id"], boolean>();
		const bookmarkAggregate = new Map<Note["id"], boolean>();
		const pinAggregate = new Map<Note["id"], boolean>();

		const renoteIds = notes
			.filter((n) => n.renoteId != null)
			.map((n) => n.renoteId!);

		const noteIds = unique(notes.map((n) => n.id));
		const targets = unique([...noteIds, ...renoteIds]);

		if (user?.id != null) {
			const mutingTargets = unique([...notes.map((n) => n.threadId ?? n.id)]);
			const pinTargets = unique([
				...notes.filter((n) => n.userId === user.id).map((n) => n.id),
			]);

			const reactions = await NoteReactions.findBy({
				userId: user.id,
				noteId: In(targets),
			});

			const renotes = await Notes.createQueryBuilder("note")
				.select("note.renoteId")
				.where("note.userId = :meId", { meId: user.id })
				.andWhere("note.renoteId IN (:...targets)", { targets })
				.andWhere("note.text IS NULL")
				.andWhere("note.hasPoll = FALSE")
				.andWhere(`note.fileIds = '{}'`)
				.getMany();

			const mutings = await NoteThreadMutings.createQueryBuilder("muting")
				.select("muting.threadId")
				.where("muting.userId = :meId", { meId: user.id })
				.andWhere("muting.threadId IN (:...targets)", {
					targets: mutingTargets,
				})
				.getMany();

			const bookmarks = await NoteFavorites.createQueryBuilder("bookmark")
				.select("bookmark.noteId")
				.where("bookmark.userId = :meId", { meId: user.id })
				.andWhere("bookmark.noteId IN (:...targets)", { targets })
				.getMany();

			const pins =
				pinTargets.length > 0
					? await UserNotePinings.createQueryBuilder("pin")
							.select("pin.noteId")
							.where("pin.userId = :meId", { meId: user.id })
							.andWhere("pin.noteId IN (:...targets)", { targets: pinTargets })
							.getMany()
					: [];

			for (const target of targets) {
				reactionAggregate.set(
					target,
					reactions.find((r) => r.noteId === target) ?? null,
				);
				renoteAggregate.set(
					target,
					!!renotes.find((n) => n.renoteId === target),
				);
				bookmarkAggregate.set(
					target,
					!!bookmarks.find((b) => b.noteId === target),
				);
			}

			for (const target of mutingTargets) {
				mutingAggregate.set(
					target,
					!!mutings.find((m) => m.threadId === target),
				);
			}

			for (const target of pinTargets) {
				mutingAggregate.set(target, !!pins.find((m) => m.noteId === target));
			}
		}

		ctx.reactionAggregate = reactionAggregate;
		ctx.renoteAggregate = renoteAggregate;
		ctx.mutingAggregate = mutingAggregate;
		ctx.bookmarkAggregate = bookmarkAggregate;
		ctx.pinAggregate = pinAggregate;

		const users = notes.filter((p) => !!p.user).map((p) => p.user as User);
		const renoteUserIds = notes
			.filter((p) => p.renoteUserId !== null)
			.map((p) => p.renoteUserId as string);
		await UserConverter.aggregateData([...users], ctx);
		await UserConverter.aggregateDataByIds(renoteUserIds, ctx);
		await prefetchEmojis(aggregateNoteEmojis(notes));
	}

	private static encodeReactions(
		reactions: Record<string, number>,
		myReaction: string | undefined,
		populated: PopulatedEmoji[],
		ctx: MastoContext,
	): MastodonEntity.Reaction[] {
		if (ctx?.tokenApp?.name === "SoraSNS for iPad") {
			// 3rd party compat
			// SoraSNS requires `reactions` to be a `Dictionary<string, int>`.
			return reactions as unknown as MastodonEntity.Reaction[];
		}
		return Object.keys(reactions)
			.map((key) => {
				const isCustom = key.startsWith(":") && key.endsWith(":");
				const name = isCustom ? key.substring(1, key.length - 1) : key;
				const populatedName =
					isCustom && name.indexOf("@") === -1 ? `${name}@.` : name;
				const url = isCustom
					? populated.find((p) => p.name === populatedName)?.url
					: undefined;

				return {
					count: reactions[key],
					me: key === myReaction,
					name: name,
					url: url,
					static_url: url,
				};
			})
			.filter((r) => r.count > 0);
	}

	public static async encodeEvent(
		note: Note,
		user: ILocalUser | undefined,
		filterContext?: string,
	): Promise<MastodonEntity.Status> {
		const ctx = getStubMastoContext(user, filterContext);
		NoteHelpers.fixupEventNote(note);
		return NoteConverter.encode(note, ctx);
	}

	public static async prewarmCache(note: Note): Promise<void> {
		if (!config.htmlCache?.prewarm) return;
		const identifier = `${note.id}:${(
			note.updatedAt ?? note.createdAt
		).getTime()}`;
		if ((await this.noteContentHtmlCache.get(identifier)) !== undefined) return;

		if (note.renoteId !== null && !note.renote) {
			note.renote = await Notes.findOneBy({ id: note.renoteId });
		}

		const quoteUri = note.renote
			? isQuote(note)
				? note.renote.url ??
				  note.renote.uri ??
				  `${config.url}/notes/${note.renote.id}`
				: null
			: null;

		const text =
			note.text !== null
				? quoteUri !== null
					? note.text
							.replaceAll(`RE: ${quoteUri}`, "")
							.replaceAll(quoteUri, "")
							.trimEnd()
					: note.text
				: null;
		const content =
			text !== null
				? MfmHelpers.toHtml(
						mfm.parse(text),
						JSON.parse(note.mentionedRemoteUsers),
						note.userHost,
						false,
						quoteUri,
				  ).then((p) => p ?? escapeMFM(text))
				: null;

		if (note.user) UserConverter.prewarmCache(note.user);
		else if (note.userId) UserConverter.prewarmCacheById(note.userId);

		if (note.replyUserId) UserConverter.prewarmCacheById(note.replyUserId);
		if (note.renoteUserId) UserConverter.prewarmCacheById(note.renoteUserId);
		this.noteContentHtmlCache.set(identifier, await content);

		// if (config.htmlCache?.dbFallback)
		//     HtmlNoteCacheEntries.upsert({ noteId: note.id, updatedAt: note.updatedAt ?? note.createdAt, content: await content }, ["noteId"]);
	}

	private static removeHash(x: string) {
		return x.replace(/#[^#]*$/, "");
	}

	private static extractUrlFromMfm(text: string | null): string[] {
		if (!text) return [];
		const nodes = mfm.parse(text);
		const urlNodes = mfm.extract(nodes, (node) => {
			return (
				node.type === "url" || (node.type === "link" && !node.props.silent)
			);
		}) as (MfmUrl | MfmLink)[];
		const urls: string[] = unique(urlNodes.map((x) => x.props.url));

		return urls.reduce((array, url) => {
			const urlWithoutHash = this.removeHash(url);
			if (!array.map((x) => this.removeHash(x)).includes(urlWithoutHash))
				array.push(url);
			return array;
		}, [] as string[]);
	}

	private static async generateCard(
		urls: string[],
		lang?: string,
	): Promise<MastodonEntity.Card | null> {
		if (urls.length === 0) return null;
		for (const url of urls) {
			try {
				const summary = await unfurl(url, {
					oembed: true,
					follow: 10,
					compress: true,
					headers: { "Accept-Language": lang ?? "en-US" },
				});
				if (summary) {
					return {
						url: summary.canonical_url ?? url,
						title: summary.title ?? "",
						description: summary.description ?? "",
						image:
							summary.oEmbed?.thumbnails?.[0]?.url ??
							summary.open_graph?.images?.[0]?.secure_url ??
							summary.open_graph?.images?.[0]?.url ??
							summary.twitter_card?.images?.[0]?.url ??
							null,
						type:
							summary.oEmbed?.type ??
							(summary.open_graph?.videos ||
							summary.open_graph?.audio ||
							summary.twitter_card?.players
								? "video"
								: "link"),
						author_name:
							summary.author ??
							summary.oEmbed?.author_name ??
							summary.open_graph?.article?.author ??
							summary.twitter_card?.creator ??
							"",
						author_url: summary.oEmbed?.author_name ?? "",
						provider_name:
							summary.oEmbed?.provider_name ??
							summary.open_graph?.site_name ??
							summary.twitter_card?.site ??
							"",
						provider_url: summary.oEmbed?.provider_url ?? "",
						html: (summary.oEmbed as { html?: string })?.html ?? "",
						width:
							summary.oEmbed?.thumbnails?.[0]?.width ??
							summary.open_graph?.images?.[0]?.width ??
							summary.open_graph?.videos?.[0]?.width ??
							0,
						height:
							summary.oEmbed?.thumbnails?.[0]?.height ??
							summary.open_graph?.images?.[0]?.height ??
							summary.open_graph?.videos?.[0]?.height ??
							0,
						embed_url:
							summary.open_graph?.videos?.[0]?.stream ??
							summary.open_graph?.videos?.[0]?.url ??
							"",
						blurhash: null,
						published_at: summary.open_graph?.article?.published_time ?? null,
						image_description: summary.open_graph?.images?.[0]?.alt ?? "",
						language: lang ?? null,
					};
				}
			} catch {
				// no op.
			}
		}
		return null;
	}
}
