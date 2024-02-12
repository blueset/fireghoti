import { nativeRandomStr } from "backend-rs";

export function secureRndstr(length = 32, _ = true): string {
	return nativeRandomStr(length);
}
