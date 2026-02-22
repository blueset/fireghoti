namespace MastodonEntity {
	export type Preferences = {
		"posting:default:visibility": "public" | "unlisted" | "private" | "direct";
		"posting:default:sensitive": boolean;
		"posting:default:language": string | null;
		"posting:default:quote_policy": "public" | "followers" | "nobody";
		"reading:expand:media": "default" | "show_all" | "hide_all";
		"reading:expand:spoilers": boolean;
	};
}
