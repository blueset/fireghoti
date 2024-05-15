import type Koa from "koa";
import { fetchMeta } from "backend-rs";
import { config } from "@/config.js";

const manifest = {
	short_name: "Firefish",
	name: "Firefish",
	description:
		"An open source, decentralized social media platform that's free forever!",
	start_url: "/",
	scope: "/",
	display: "standalone",
	background_color: "#1f1d2e",
	theme_color: "#31748f",
	orientation: "natural",
	icons: [
		{
			src: "/static-assets/icons/192.png",
			sizes: "192x192",
			type: "image/png",
			purpose: "any",
		},
		{
			src: "/static-assets/icons/512.png",
			sizes: "512x512",
			type: "image/png",
			purpose: "any",
		},
		{
			src: "/static-assets/icons/maskable.png",
			sizes: "512x512",
			type: "image/png",
			purpose: "maskable",
		},
		{
			src: "/static-assets/icons/monochrome.png",
			sizes: "512x512",
			type: "image/png",
			purpose: "monochrome",
		},
	],
	share_target: {
		action: "/share/",
		params: {
			title: "title",
			text: "text",
			url: "url",
		},
	},
	screenshots: [
		{
			src: "/static-assets/screenshots/1.webp",
			sizes: "1080x2340",
			type: "image/webp",
			platform: "narrow",
			label: "Profile page",
		},
		{
			src: "/static-assets/screenshots/2.webp",
			sizes: "1080x2340",
			type: "image/webp",
			platform: "narrow",
			label: "Posts",
		},
	],
	shortcuts: [
		{
			name: "Notifications",
			short_name: "Notifs",
			url: "/my/notifications",
		},
		{
			name: "Chats",
			url: "/my/messaging",
		},
	],
	categories: ["social"],
};

export const manifestHandler = async (ctx: Koa.Context) => {
	const instance = await fetchMeta(true);

	manifest.short_name = instance.name || "Firefish";
	manifest.name = instance.name || "Firefish";
	if (instance.themeColor) manifest.theme_color = instance.themeColor;
	for (const icon of manifest.icons) {
		icon.src = `${icon.src}?v=${config.version.replace(/[^0-9]/g, "")}`;
	}
	for (const screenshot of manifest.screenshots) {
		screenshot.src = `${screenshot.src}?v=${config.version.replace(
			/[^0-9]/g,
			"",
		)}`;
	}
	ctx.set("Cache-Control", "max-age=300");
	ctx.body = manifest;
};
