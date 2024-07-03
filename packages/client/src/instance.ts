import type { entities } from "firefish-js";
import { computed } from "vue";
import { api } from "./os";
import { set, get } from "idb-keyval";

// TODO: 他のタブと永続化されたstateを同期

// TODO: get("instance") requires top-level await
let instance: entities.DetailedInstanceMetadata;

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

	// TODO: set default values
	instance = {} as entities.DetailedInstanceMetadata;

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
