import path, { join } from "node:path";
import { fileURLToPath } from "node:url";
import { execa } from "execa";

(async () => {
	const __dirname = path.dirname(fileURLToPath(import.meta.url));

	execa("cargo", ["clean"], {
		cwd: join(__dirname, "/../packages/backend/native-utils"),
		stdio: "inherit",
	});
})();
