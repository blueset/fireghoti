import { MfmHelpers } from "@/server/api/mastodon/helpers/mfm.js";
import mfm from "mfm-js";
import type { Announcement } from "@/models/entities/announcement.js";
import type { MastoContext } from "..";

export class AnnouncementConverter {
	public static async encode(
		announcement: Announcement,
		isRead: boolean,
		ctx: MastoContext,
	): Promise<MastodonEntity.Announcement> {
		return {
			id: announcement.id,
			content: `<h1>${
				(await MfmHelpers.toHtml(mfm.parse(announcement.title), [], null, false, null, ctx)) ??
				"Announcement"
			}</h1>${
				(await MfmHelpers.toHtml(mfm.parse(announcement.text), [], null, false, null, ctx)) ?? ""
			}`,
			starts_at: null,
			ends_at: null,
			published: true,
			all_day: false,
			published_at: announcement.createdAt.toISOString(),
			updated_at:
				announcement.updatedAt?.toISOString() ??
				announcement.createdAt.toISOString(),
			read: isRead,
			mentions: [], //FIXME
			statuses: [],
			tags: [],
			emojis: [], //FIXME
			reactions: [],
		};
	}
}
