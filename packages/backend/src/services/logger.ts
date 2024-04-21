import cluster from "node:cluster";
import chalk from "chalk";
import { default as convertColor } from "color-convert";
import { format as dateFormat } from "date-fns";
import config, { envOption } from "@/config/index.js";

import * as SyslogPro from "syslog-pro";

type Domain = {
	name: string;
	color?: string;
};

type Level = "error" | "success" | "warning" | "debug" | "info";

export default class Logger {
	private domain: Domain;
	private parentLogger: Logger | null = null;
	private store: boolean;
	private syslogClient: any | null = null;

	constructor(domain: string, color?: string, store = true) {
		this.domain = {
			name: domain,
			color: color,
		};
		this.store = store;

		if (config.syslog) {
			this.syslogClient = new SyslogPro.RFC5424({
				applicationName: "Firefish",
				timestamp: true,
				includeStructuredData: true,
				color: true,
				extendedColor: true,
				server: {
					target: config.syslog.host,
					port: config.syslog.port,
				},
			});
		}
	}

	public createSubLogger(domain: string, color?: string, store = true): Logger {
		const logger = new Logger(domain, color, store);
		logger.parentLogger = this;
		return logger;
	}

	private log(
		level: Level,
		message: string,
		data?: Record<string, any> | null,
		important = false,
		subDomains: Domain[] = [],
		store = true,
	): void {
		if (
			!(typeof config.logLevel === "undefined") &&
			!config.logLevel.includes(level)
		)
			return;
		if (!this.store) store = false;
		if (level === "debug") store = false;

		if (this.parentLogger) {
			this.parentLogger.log(
				level,
				message,
				data,
				important,
				[this.domain].concat(subDomains),
				store,
			);
			return;
		}

		const time = dateFormat(new Date(), "HH:mm:ss");
		const worker = cluster.isPrimary ? "*" : cluster.worker.id;
		const l =
			level === "error"
				? important
					? chalk.bgRed.white("ERR ")
					: chalk.red("ERR ")
				: level === "warning"
					? chalk.yellow("WARN")
					: level === "success"
						? important
							? chalk.bgGreen.white("DONE")
							: chalk.green("DONE")
						: level === "debug"
							? chalk.gray("VERB")
							: level === "info"
								? chalk.blue("INFO")
								: null;
		const domains = [this.domain]
			.concat(subDomains)
			.map((d) =>
				d.color
					? chalk.rgb(...convertColor.keyword.rgb(d.color))(d.name)
					: chalk.white(d.name),
			);
		const m =
			level === "error"
				? chalk.red(message)
				: level === "warning"
					? chalk.yellow(message)
					: level === "success"
						? chalk.green(message)
						: level === "debug"
							? chalk.gray(message)
							: level === "info"
								? message
								: null;

		let log = `${l} ${worker}\t[${domains.join(" ")}]\t${m}`;
		if (envOption.withLogTime) log = `${chalk.gray(time)} ${log}`;

		console.log(important ? chalk.bold(log) : log);

		if (level === "error" && data != null) {
			console.log(data);
		}

		if (store) {
			if (this.syslogClient) {
				const send =
					level === "error"
						? this.syslogClient.error
						: level === "warning"
							? this.syslogClient.warning
							: level === "success"
								? this.syslogClient.info
								: level === "debug"
									? this.syslogClient.info
									: level === "info"
										? this.syslogClient.info
										: (null as never);

				send
					.bind(this.syslogClient)(message)
					.catch(() => {});
			}
		}
	}

	// Used when the process can't continue (fatal error)
	public error(
		x: string | Error,
		data?: Record<string, any> | null,
		important = false,
	): void {
		if (x instanceof Error) {
			data = data || {};
			data.e = x;
			this.log("error", x.toString(), data, important);
		} else if (typeof x === "object") {
			this.log(
				"error",
				`${(x as any).message || (x as any).name || x}`,
				data,
				important,
			);
		} else {
			this.log("error", `${x}`, data, important);
		}
	}

	// Used when the process can continue but some action should be taken
	public warn(
		message: string,
		data?: Record<string, any> | null,
		important = false,
	): void {
		this.log("warning", message, data, important);
	}

	// Used when something is successful
	public succ(
		message: string,
		data?: Record<string, any> | null,
		important = false,
	): void {
		this.log("success", message, data, important);
	}

	// Used for debugging (information necessary for developers but unnecessary for users)
	public debug(
		message: string,
		data?: Record<string, any> | null,
		important = false,
	): void {
		// Fixed if statement is ignored when logLevel includes debug
		if (
			config.logLevel?.includes("debug") ||
			process.env.NODE_ENV !== "production" ||
			envOption.verbose
		) {
			this.log("debug", message, data, important);
		}
	}

	// Other generic logs
	public info(
		message: string,
		data?: Record<string, any> | null,
		important = false,
	): void {
		this.log("info", message, data, important);
	}
}
