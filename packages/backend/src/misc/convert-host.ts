import { URL } from "node:url";
import config from "@/config/index.js";
import { toASCII } from "punycode";
import Logger from "@/services/logger.js";
import { inspect } from "node:util";

const logger = new Logger("convert-host");

export function getFullApAccount(username: string, host: string | null) {
	return host
		? `${username}@${toPuny(host)}`
		: `${username}@${toPuny(config.host)}`;
}

export function isSelfHost(host: string) {
	if (host == null) return true;
	return toPuny(config.host) === toPuny(host);
}

export function isSameOrigin(src: unknown): boolean | null {
	if (typeof src !== "string") {
		logger.debug(`unknown origin: ${inspect(src)}`);
		return null;
	}
	try {
		const u = new URL(src);
		return u.origin === config.url;
	} catch (e) {
		logger.debug(inspect(e));
		return false;
	}
}

export function extractDbHost(uri: string) {
	const url = new URL(uri);
	return toPuny(url.hostname);
}

export function toPuny(host: string) {
	return toASCII(host.toLowerCase());
}

export function toPunyNullable(host: string | null | undefined): string | null {
	if (host == null) return null;
	return toASCII(host.toLowerCase());
}
