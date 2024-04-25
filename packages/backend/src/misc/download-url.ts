import * as fs from "node:fs";
import * as stream from "node:stream/promises";
import got, * as Got from "got";
import { config } from "@/config.js";
import { getAgentByHostname, StatusError } from "./fetch.js";
import chalk from "chalk";
import Logger from "@/services/logger.js";
import IPCIDR from "ip-cidr";
import PrivateIp from "private-ip";
import { isValidUrl } from "./is-valid-url.js";

export async function downloadUrl(url: string, path: string): Promise<void> {
	if (!isValidUrl(url)) {
		throw new StatusError("Invalid URL", 400);
	}

	const downloadLogger = new Logger("download");

	downloadLogger.debug(`Downloading ${chalk.cyan(url)} ...`);

	const timeout = 30 * 1000;
	const operationTimeout = 60 * 1000;
	const maxSize = config.maxFileSize || 262144000;

	const req = got
		.stream(url, {
			headers: {
				"User-Agent": config.userAgent,
				Host: new URL(url).hostname,
			},
			timeout: {
				lookup: timeout,
				connect: timeout,
				secureConnect: timeout,
				socket: timeout, // read timeout
				response: timeout,
				send: timeout,
				request: operationTimeout, // whole operation timeout
			},
			agent: getAgentByHostname(new URL(url).hostname),
			http2: false, // default
			retry: {
				limit: 0,
			},
		})
		.on("redirect", (res: Got.Response, opts: Got.NormalizedOptions) => {
			if (!isValidUrl(opts.url)) {
				downloadLogger.warn(`Invalid URL: ${opts.url}`);
				req.destroy();
			}
		})
		.on("response", (res: Got.Response) => {
			if (
				(process.env.NODE_ENV === "production" ||
					process.env.NODE_ENV === "test") &&
				!config.proxy &&
				res.ip
			) {
				if (isPrivateIp(res.ip)) {
					downloadLogger.warn(`Blocked address: ${res.ip}`);
					req.destroy();
				}
			}

			const contentLength = res.headers["content-length"];
			if (contentLength != null) {
				const size = Number(contentLength);
				if (size > maxSize) {
					downloadLogger.warn(
						`maxSize exceeded (${size} > ${maxSize}) on response`,
					);
					req.destroy();
				}
			}
		})
		.on("downloadProgress", (progress: Got.Progress) => {
			if (progress.transferred > maxSize) {
				downloadLogger.warn(
					`maxSize exceeded (${progress.transferred} > ${maxSize}) on downloadProgress`,
				);
				req.destroy();
			}
		});

	try {
		await stream.pipeline(req, fs.createWriteStream(path));
	} catch (e) {
		if (e instanceof Got.HTTPError) {
			throw new StatusError(
				`${e.response.statusCode} ${e.response.statusMessage}`,
				e.response.statusCode,
				e.response.statusMessage,
			);
		} else {
			throw e;
		}
	}

	downloadLogger.debug(`Download finished: ${chalk.cyan(url)}`);
}

export function isPrivateIp(ip: string): boolean {
	for (const net of config.allowedPrivateNetworks || []) {
		const cidr = new IPCIDR(net);
		if (cidr.contains(ip)) {
			return false;
		}
	}

	return PrivateIp(ip);
}
