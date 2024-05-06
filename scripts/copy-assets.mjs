import fs from "node:fs/promises";
import path, { join } from "node:path";
import { fileURLToPath } from "node:url";

const repositoryRootDir = join(path.dirname(fileURLToPath(import.meta.url)), "../");
const file = (relativePath) => join(repositoryRootDir, relativePath);

await (async () => {
	await fs.rm(file("built/_client_dist_/locales"), { recursive: true, force: true });
	await Promise.all([
		fs.cp(file("packages/backend/src/server/web"), file("packages/backend/built/server/web"), { recursive: true }),
		fs.cp(file("custom/assets"), file("packages/backend/assets"), { recursive: true }),
		fs.cp(file("packages/client/node_modules/three/examples/fonts"), file("built/_client_dist_/fonts"), { recursive: true }),
		fs.mkdir(file("built/_client_dist_/locales"), { recursive: true }),
	]);

	const locales = (await import("../locales/index.mjs")).default;
	const meta = (await import("../built/meta.json", { assert: { type: "json" } })).default;

	for await (const [lang, locale] of Object.entries(locales)) {
		await fs.writeFile(
			file(`built/_client_dist_/locales/${lang}.${meta.version}.json`),
			JSON.stringify({ ...locale, _version_: meta.version }),
			"utf-8",
		);
	}

	const js_assets = [
		file("packages/backend/built/server/web/boot.js"),
		file("packages/backend/built/server/web/bios.js"),
		file("packages/backend/built/server/web/cli.js"),
	];

	for await (const js_file of js_assets) {
		const content = (await fs.readFile(js_file, "utf-8"))
			.replace("SUPPORTED_LANGS", JSON.stringify(Object.keys(locales)));
		await fs.writeFile(js_file, content, "utf-8");
	}

	// TODO?: minify packages/backend/built/server/web/*.css
})();
