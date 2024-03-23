import { lang } from "@/config";

export const versatileLang = (lang ?? "ja-JP").replace("ja-KS", "ja-JP").replace("en-US", "en-GB");
export const dateTimeFormat = new Intl.DateTimeFormat(versatileLang, {
	year: "numeric",
	month: "short",
	day: "numeric",
	hour: "numeric",
	minute: "numeric",
	second: "numeric",
	hourCycle: "h23",
	weekday: "short",
	timeZoneName: "shortOffset",
});
export const numberFormat = new Intl.NumberFormat(versatileLang);
