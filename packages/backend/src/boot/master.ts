import * as fs from "node:fs";
import { fileURLToPath } from "node:url";
import { dirname } from "node:path";
import * as os from "node:os";
import cluster from "node:cluster";
import chalk from "chalk";
import chalkTemplate from "chalk-template";
import semver from "semver";

import Logger from "@/services/logger.js";
import type { Config } from "backend-rs";
import { initializeRustLogger } from "backend-rs";
import { fetchMeta, removeOldAttestationChallenges } from "backend-rs";
import { config, envOption } from "@/config.js";
import { showMachineInfo } from "@/misc/show-machine-info.js";
import { db, initDb } from "@/db/postgre.js";
import { inspect } from "node:util";

const _filename = fileURLToPath(import.meta.url);
const _dirname = dirname(_filename);

const meta = JSON.parse(
	fs.readFileSync(`${_dirname}/../../../../built/meta.json`, "utf-8"),
);

const logger = new Logger("core", "cyan");
const bootLogger = logger.createSubLogger("boot", "magenta", false);

const themeColor = chalk.hex("#31748f");

function greet() {
	//#region Firefish logo
	console.log(
		themeColor(
			"██████╗ ██╗██████╗ ███████╗███████╗██╗███████╗██╗  ██╗    ○     ▄    ▄    ",
		),
	);
	console.log(
		themeColor(
			"██╔════╝██║██╔══██╗██╔════╝██╔════╝██║██╔════╝██║  ██║      ⚬   █▄▄  █▄▄ ",
		),
	);
	console.log(
		themeColor(
			"█████╗  ██║██████╔╝█████╗  █████╗  ██║███████╗███████║      ▄▄▄▄▄▄   ▄    ",
		),
	);
	console.log(
		themeColor(
			"██╔══╝  ██║██╔══██╗██╔══╝  ██╔══╝  ██║╚════██║██╔══██║     █      █  █▄▄  ",
		),
	);
	console.log(
		themeColor(
			"██║     ██║██║  ██║███████╗██║     ██║███████║██║  ██║     █ ● ●  █       ",
		),
	);
	console.log(
		themeColor(
			"╚═╝     ╚═╝╚═╝  ╚═╝╚══════╝╚═╝     ╚═╝╚══════╝╚═╝  ╚═╝     ▀▄▄▄▄▄▄▀       ",
		),
	);
	//#endregion

	console.log(
		" Firefish is an open-source decentralized microblogging platform.",
	);
	console.log(
		chalk.rgb(
			255,
			136,
			0,
		)(
			" If you like Firefish, please consider contributing to the repo. https://firefish.dev/firefish/firefish",
		),
	);

	console.log("");
	console.log(
		chalkTemplate`--- ${os.hostname()} {gray (PID: ${process.pid.toString()})} ---`,
	);

	bootLogger.info("Welcome to Firefish!");
	bootLogger.info(`Firefish v${meta.version}`, null, true);
}

/**
 * Init master process
 */
export async function masterMain() {
	// initialize app
	try {
		greet();
		showEnvironment();
		await showMachineInfo(bootLogger);
		showNodejsVersion();
		await connectDb();
		initializeRustLogger();
	} catch (e) {
		bootLogger.error(
			`Fatal error occurred during initialization:\n${inspect(e)}`,
			null,
			true,
		);
		process.exit(1);
	}

	bootLogger.info("Firefish initialized");

	if (!envOption.disableClustering) {
		await spawnWorkers(config.clusterLimits);
	}

	bootLogger.info(
		`Now listening on port ${config.port} on ${config.url}`,
		null,
		true,
	);

	if (!envOption.noDaemons) {
		import("../daemons/server-stats.js").then((x) => x.default());
		import("../daemons/queue-stats.js").then((x) => x.default());
		// Update meta cache every 5 minitues
		setInterval(() => fetchMeta(false), 1000 * 60 * 5);
		// Remove old attestation challenges
		setInterval(() => removeOldAttestationChallenges(), 1000 * 60 * 30);
	}
}

function showEnvironment(): void {
	const env = process.env.NODE_ENV;
	const logger = bootLogger.createSubLogger("env");
	logger.info(
		typeof env === "undefined" ? "NODE_ENV is not set" : `NODE_ENV: ${env}`,
	);

	if (env !== "production") {
		logger.warn("The environment is not in production mode.");
		logger.warn("DO NOT USE THIS IN PRODUCTION!", null, true);
	}
}

function showNodejsVersion(): void {
	const nodejsLogger = bootLogger.createSubLogger("nodejs");

	nodejsLogger.info(`Version ${process.version} detected.`);

	const minVersion = "v18.17.0";
	if (semver.lt(process.version, minVersion)) {
		nodejsLogger.error(`At least Node.js ${minVersion} required!`);
		process.exit(1);
	}
}

async function connectDb(): Promise<void> {
	const dbLogger = bootLogger.createSubLogger("db");

	// Try to connect to DB
	try {
		dbLogger.info("Connecting to the database...");
		await initDb();
		const v = await db
			.query("SHOW server_version")
			.then((x) => x[0].server_version);
		dbLogger.info(`Connected: v${v}`);
	} catch (e) {
		dbLogger.error("Failed to connect to the database", null, true);
		dbLogger.error(inspect(e));
		process.exit(1);
	}
}

async function spawnWorkers(
	clusterLimits: Config["clusterLimits"],
): Promise<void> {
	const cpus = os.cpus().length;

	if (clusterLimits.queue > cpus) {
		bootLogger.warn(
			"config: queue cluster limit exceeds the number of cpu cores",
		);
	}

	if (clusterLimits.web > cpus) {
		bootLogger.warn(
			"config: web cluster limit exceeds the number of cpu cores",
		);
	}

	const total = clusterLimits.queue + clusterLimits.web;

	// workers = ["web", "web", ..., "web", "queue", "queue", ..., "queue"]
	const workers = new Array(total);
	workers.fill("web", 0, clusterLimits.web);
	workers.fill("queue", clusterLimits.web);

	bootLogger.info(
		`Starting ${clusterLimits.web} web workers and ${clusterLimits.queue} queue workers (total ${total})...`,
	);
	await Promise.all(workers.map((mode) => spawnWorker(mode)));
	bootLogger.info("All workers started");
}

function spawnWorker(mode: "web" | "queue"): Promise<void> {
	return new Promise((res) => {
		const worker = cluster.fork({ mode });
		worker.on("message", (message) => {
			if (message === "listenFailed") {
				bootLogger.error("The server listen failed due to the previous error.");
				process.exit(1);
			}
			if (message !== "ready") return;
			res();
		});
	});
}
