import cluster from "node:cluster";
import { config } from "@/config.js";
import { initDb } from "@/db/postgre.js";
import { initIdGenerator } from "backend-rs";
import os from "node:os";

/**
 * Init worker process
 */
export async function workerMain() {
	const length = Math.min(Math.max(config.cuid?.length ?? 16, 16), 24);
	const fingerprint = config.cuid?.fingerprint ?? "";
	initIdGenerator(length, fingerprint);

	await initDb();

	if (!process.env.mode || process.env.mode === "web") {
		// start server
		await import("../server/index.js").then((x) => x.default());
	}

	if (!process.env.mode || process.env.mode === "queue") {
		// start job queue
		import("../queue/index.js").then((x) => x.default());

		if (process.env.mode === "queue") {
			// if this is an exclusive queue worker, renice to have higher priority
			os.setPriority(os.constants.priority.PRIORITY_BELOW_NORMAL);
		}
	}

	if (cluster.isWorker) {
		// Send a 'ready' message to parent process
		process.send!("ready");
	}
}
