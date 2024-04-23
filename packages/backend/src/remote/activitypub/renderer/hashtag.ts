import { config } from "@/config.js";

export default (tag: string) => ({
	type: "Hashtag",
	href: `${config.url}/tags/${encodeURIComponent(tag)}`,
	name: `#${tag}`,
});
