import fs from "node:fs";
import path, { join } from "node:path";
import { fileURLToPath } from "node:url";
import { execa } from "execa";

(async () => {
	const __dirname = path.dirname(fileURLToPath(import.meta.url));

	await execa(
		"pnpm", [
			"--recursive",
			"--parallel",
			"--filter=backend-rs",
			"--filter=firefish-js",
			"run",
			"build",
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
			"build",
		], {
			cwd: join(__dirname, "/../"),
			stdio: "inherit",
		}
	);

	fs.copyFileSync("packages/backend-rs/index.js", "packages/backend-rs/built/index.js");
})();
