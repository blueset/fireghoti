import { Packed } from "@/misc/schema.js";

export class FileConverter {
	public static encode(f: Packed<"DriveFile">): MastodonEntity.Attachment {
		return {
			id: f.id,
			type: this.encodefileType(f.type),
			url: f.url ?? "",
			remote_url: f.url,
			preview_url: f.thumbnailUrl ?? "", // fix compat: Mastodon for Android does not accept null preview_url
			text_url: f.url,
			meta: {
				width: f.properties.width,
				height: f.properties.height,
				original: {
					width: f.properties.width,
					height: f.properties.height,
					size: f.properties.width && f.properties.height ? `${f.properties.width}x${f.properties.height}` : undefined,
					aspect: f.properties.width && f.properties.height ? f.properties.width / f.properties.height : undefined,
				}
			},
			description: f.comment,
			blurhash: f.blurhash,
		};
	}

	private static encodefileType(
		s: string,
	): "unknown" | "image" | "gifv" | "video" | "audio" {
		if (s === "image/gif") {
			return "gifv";
		}
		if (s.includes("image")) {
			return "image";
		}
		if (s.includes("video")) {
			return "video";
		}
		if (s.includes("audio")) {
			return "audio";
		}
		return "unknown";
	}
}
