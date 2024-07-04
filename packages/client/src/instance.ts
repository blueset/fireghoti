import type { entities } from "firefish-js";
import { computed } from "vue";
import { api } from "./os";
import { set, get } from "idb-keyval";

// TODO: 他のタブと永続化されたstateを同期

// TODO: get("instance") requires top-level await
// TODO: fallback to defaults more nicely
// default values
let instance: entities.DetailedInstanceMetadata = {
	maintainerName: "",
	maintainerEmail: "",
	version: "",
	name: null,
	uri: "",
	tosUrl: null,
	description: null,
	disableRegistration: true,
	disableLocalTimeline: false,
	disableGlobalTimeline: false,
	disableRecommendedTimeline: true,
	enableGuestTimeline: false,
	driveCapacityPerLocalUserMb: 1000,
	driveCapacityPerRemoteUserMb: 0,
	antennaLimit: 5,
	enableHcaptcha: false,
	hcaptchaSiteKey: null,
	enableRecaptcha: false,
	recaptchaSiteKey: null,
	swPublickey: null,
	maxNoteTextLength: 3000,
	maxCaptionTextLength: 1500,
	enableEmail: false,
	enableServiceWorker: false,
	markLocalFilesNsfwByDefault: false,
	emojis: [],
	ads: [],
	langs: [],
	moreUrls: {},
	repositoryUrl: "https://firefish.dev/firefish/firefish",
	feedbackUrl: "https://firefish.dev/firefish/firefish/-/issues",
	defaultDarkTheme: null,
	defaultLightTheme: null,
	defaultReaction: "⭐",
	cacheRemoteFiles: false,
	proxyAccountName: null,
	emailRequiredForSignup: false,
	mascotImageUrl: "",
	bannerUrl: "",
	backgroundImageUrl: "",
	errorImageUrl: "",
	iconUrl: null,
	requireSetup: false,
	translatorAvailable: false,
	features: {
		registration: false,
		localTimeLine: true,
		recommendedTimeLine: false,
		globalTimeLine: true,
		searchFilters: true,
		hcaptcha: false,
		recaptcha: false,
		objectStorage: false,
		serviceWorker: false,
	},
};

export function getInstanceInfo(): entities.DetailedInstanceMetadata {
	return instance;
}

export async function initializeInstanceCache(): Promise<void> {
	// Is the data stored in IndexDB?
	const fromIdb = await get<string>("instance");
	if (fromIdb != null) {
		instance = JSON.parse(fromIdb);
	}
	// Call API
	updateInstanceCache();
}

export async function updateInstanceCache(): Promise<void> {
	const meta = await api("meta", {
		detail: true,
	});

	for (const [k, v] of Object.entries(meta)) {
		instance[k] = v;
	}
	set("instance", JSON.stringify(instance));
}

export const emojiCategories = computed(() => {
	if (instance.emojis == null) return [];
	const categories = new Set();
	for (const emoji of instance.emojis) {
		categories.add(emoji.category);
	}
	return Array.from(categories);
});

export const emojiTags = computed(() => {
	if (instance.emojis == null) return [];
	const tags = new Set();
	for (const emoji of instance.emojis) {
		for (const tag of emoji.aliases) {
			tags.add(tag);
		}
	}
	return Array.from(tags);
});
