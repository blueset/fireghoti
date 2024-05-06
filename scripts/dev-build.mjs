import path, { join } from "node:path";
import { fileURLToPath } from "node:url";
import { execa } from "execa";
import fs from "node:fs";

(async () => {
	const __dirname = path.dirname(fileURLToPath(import.meta.url));

	await execa(
		"pnpm", [
			"--recursive",
			"--parallel",
			"--filter=backend-rs",
			"--filter=firefish-js",
			"run",
			"build:debug",
		], {
			cwd: join(__dirname, "/../"),
			stdio: "inherit",
		}
	);

	await execa(
		"pnpm",	[
			"--recursive",
			"--parallel",
			"--filter=!backend-rs",
			"--filter=!firefish-js",
			"run",
			"build:debug",
		], {
			cwd: join(__dirname, "/../"),
			stdio: "inherit",
		}
	);

	if (!fs.existsSync(join(__dirname, "/../packages/backend-rs/built/index.js"))) {
		fs.copyFileSync(
			join(__dirname, "/../packages/backend-rs/index.js"),
			join(__dirname, "/../packages/backend-rs/built/index.js"),
		);
		console.warn("backend-rs/built/index.js has not been updated (https://github.com/napi-rs/napi-rs/issues/1768)");
	}
	if (!fs.existsSync(join(__dirname, "/../packages/backend-rs/built/index.d.ts"))) {
		fs.copyFileSync(
			join(__dirname, "/../packages/backend-rs/index.d.ts"),
			join(__dirname, "/../packages/backend-rs/built/index.d.ts"),
		);
	}
})();
