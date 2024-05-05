import fs from "node:fs/promises";

await (async () => {
	await fs.rm("built/_client_dist_/locales", { recursive: true, force: true });
	await Promise.all([
		fs.cp("packages/backend/src/server/web", "packages/backend/built/server/web", { recursive: true }),
		fs.cp("custom/assets", "packages/backend/assets", { recursive: true }),
		fs.cp("packages/client/node_modules/three/examples/fonts", "built/_client_dist_/fonts", { recursive: true }),
		fs.mkdir("built/_client_dist_/locales", { recursive: true }),
	]);

	const locales = (await import("../locales/index.mjs")).default;
	const meta = (await import("../built/meta.json", { assert: { type: "json" } })).default;

	for await (const [lang, locale] of Object.entries(locales)) {
		await fs.writeFile(
			`built/_client_dist_/locales/${lang}.${meta.version}.json`,
			JSON.stringify({ ...locale, _version_: meta.version }),
			"utf-8",
		);
	}

	const js_assets = [
		"packages/backend/built/server/web/boot.js",
		"packages/backend/built/server/web/bios.js",
		"packages/backend/built/server/web/cli.js",
	];

	for await (const js_file of js_assets) {
		const content = (await fs.readFile(js_file, "utf-8"))
			.replace("SUPPORTED_LANGS", JSON.stringify(Object.keys(locales)));
		await fs.writeFile(js_file, content, "utf-8");
	}

	// TODO?: minify packages/backend/built/server/web/*.css
})();
