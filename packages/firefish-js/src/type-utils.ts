import type { Endpoints } from "./api.types";

export type Equal<X, Y> = (<T>() => T extends X ? 1 : 2) extends <
	T,
>() => T extends Y ? 1 : 2
	? true
	: false;

export type PropertyOfType<Type, U> = {
	[K in keyof Type]: Type[K] extends U ? K : never;
}[keyof Type];

export type EndpointsOf<T> = PropertyOfType<Endpoints, { res: T }>;
