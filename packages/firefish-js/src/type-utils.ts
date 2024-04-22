import type { Endpoints } from "./api.types";

export type PropertyOfType<Type, U> = {
	[K in keyof Type]: Type[K] extends U ? K : never;
}[keyof Type];

export type EndpointsOf<T> = PropertyOfType<Endpoints, { res: T }>;
