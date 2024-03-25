import { type HTMLAnchorElement, type HTMLLinkElement, Window } from "happy-dom";
import config from "@/config/index.js";
import { getHtml } from "@/misc/fetch.js";

async function getRelMeLinks(url: string): Promise<string[]> {
	try {
		// const html = await getHtml(url);
		const dom = new Window({
			url: url,
			settings: {
				disableJavaScriptEvaluation: true,
				disableJavaScriptFileLoading: true,
				disableCSSFileLoading: true,
				disableComputedStyleRendering: true,
				disableIframePageLoading: true,
			}
		});
		dom.document.open();
		dom.document.write(await getHtml(url));
		dom.document.close();
		// await new Promise<void>((resolve) => dom.document.readyState === "complete" ? resolve() : dom.document.addEventListener("load", () => resolve()));
		// await dom.happyDOM.waitUntilComplete();
		const allLinks = [...dom.window.document.querySelectorAll("a, link")];
		const relMeLinks = allLinks
			.filter((a) => {
				const relAttribute = a.getAttribute("rel");
				return relAttribute ? relAttribute.split(" ").includes("me") : false;
			})
			.map((a) => (a as HTMLAnchorElement | HTMLLinkElement).href);
		return relMeLinks;
	} catch {
		return [];
	}
}

export async function verifyLink(
	link: string,
	username?: string,
	url?: string | null,
): Promise<boolean> {
	let verified = false;
	if (link.startsWith("http")) {
		const relMeLinks = await getRelMeLinks(link);
		verified = relMeLinks.some((href) =>
			(url && href.startsWith(url)) ||
			(username && new RegExp(
				`^https?:\/\/${config.host.replace(
					/[.*+\-?^${}()|[\]\\]/g,
					"\\$&",
				)}\/@${username.replace(/[.*+\-?^${}()|[\]\\]/g, "\\$&")}$`,
			).test(href)),
		);
	}
	return verified;
}
