import { nativeRandomStr } from "backend-rs/built/index.js";

export function secureRndstr(length = 32, _ = true): string {
	return nativeRandomStr(length);
}
