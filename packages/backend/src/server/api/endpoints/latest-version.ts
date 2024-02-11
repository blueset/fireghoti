import define from "@/server/api/define.js";

export const meta = {
	tags: ["meta"],

	requireCredential: false,
	requireCredentialPrivateMode: true,
} as const;

export const paramDef = {
	type: "object",
	properties: {},
	required: [],
} as const;

export default define(meta, paramDef, async () => {
	let latest_version;
	await fetch("https://firefish.dev/firefish/firefish/-/raw/main/package.json")
		.then((response) => response.json())
		.then((data) => {
			latest_version = data.version;
		});

	return {
		latest_version,
	};
});
