import path, { join } from "node:path";
import { fileURLToPath } from "node:url";
import { execa } from "execa";

(async () => {
	const __dirname = path.dirname(fileURLToPath(import.meta.url));

	// from dev/docker-compose.yml
	const POSTGRES_USER = "firefish";
	const POSTGRES_PASSWORD = "password";
	const POSTGRES_DB = "firefish_db";
	const POSTGRES_PORT = "25432";

	await execa("pnpm", ["run", "migrate"], {
		cwd: join(__dirname, "/.."),
		stdio: "inherit",
	});

	await execa("sea-orm-cli", [
			"generate",
			"entity",
			"--output-dir=src/model/entity",
			`--database-url=postgres://${POSTGRES_USER}:${POSTGRES_PASSWORD}@localhost:${POSTGRES_PORT}/${POSTGRES_DB}`,
		], {
		cwd: join(__dirname, "/../packages/backend-rs"),
		stdio: "inherit",
	});
})();
