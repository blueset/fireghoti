import { Middleware } from "@koa/router";
import { HttpMethodEnum, koaBody } from "koa-body";

interface Part {
	name: string | null;
	originalFilename: string | null;
	mimetype: string | null;
}

export function KoaBodyMiddleware(): Middleware {
	const options = {
		multipart: true,
		urlencoded: true,
		parsedMethods: [
			HttpMethodEnum.POST,
			HttpMethodEnum.PUT,
			HttpMethodEnum.PATCH,
			HttpMethodEnum.DELETE,
		], // dear god mastodon why
	};

	return koaBody(options);
}
