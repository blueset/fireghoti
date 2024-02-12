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
})();
