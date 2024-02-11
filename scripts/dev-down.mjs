import path, { join } from "node:path";
import { fileURLToPath } from "node:url";
import { execa } from "execa";

(async () => {
	const __dirname = path.dirname(fileURLToPath(import.meta.url));

	execa("podman-compose", ["down"], {
		cwd: join(__dirname, "/../dev"),
		stdio: "inherit",
	});
})();
