import type {
	FoldableNotification,
	NotificationFolded,
} from "@/types/notification";
import type { entities } from "firefish-js";

interface FoldOption {
	/** If items length is 1, skip aggregation */
	/** @default true */
	skipSingleElement?: boolean;
}

/**
 * Fold similar content
 * @param ns items to fold
 * @param fetch_limit fetch limit of pagination. items will be divided into subarrays with this limit as the length.
 * @param classfier Classify the given item into a certain category (return a string representing the category)
 * @param aggregator Aggregate items of the given class into itemfolded
 * @returns folded items
 */
export function foldItems<ItemFolded, Item>(
	ns: Item[],
	classfier: (n: Item, index: number) => string,
	aggregator: (ns: Item[], key: string) => ItemFolded,
	_options?: FoldOption,
) {
	let res: (ItemFolded | Item)[] = [];

	const options: FoldOption = _options ?? {};
	options.skipSingleElement ??= true;

	const toAppendKeys: string[] = [];
	const foldMap = new Map<string, Item[]>();

	for (const [index, n] of ns.entries()) {
		const key = classfier(n, index);
		const arr = foldMap.get(key);
		if (arr != null) {
			arr.push(n);
		} else {
			foldMap.set(key, [n]);
			toAppendKeys.push(key);
		}
	}

	res = toAppendKeys.map((key) => {
		const arr = foldMap.get(key)!;
		if (arr?.length === 1 && options?.skipSingleElement) {
			return arr[0];
		}
		return aggregator(arr, key);
	});

	return res;
}

export function foldNotifications(ns: entities.Notification[]) {
	// By the implement of MkPagination, lastId is unique and is safe for key
	const lastId = ns[ns.length - 1]?.id ?? "prepend";
	return foldItems(
		ns,
		(n) => {
			switch (n.type) {
				case "renote":
					return `renote-${n.note.renote.id}`;
				case "reaction":
					return `reaction-${n.reaction}-of-${n.note.id}`;
				default: {
					return `${n.id}`;
				}
			}
		},
		(ns, key) => {
			const represent = ns[0];
			function check(
				ns: entities.Notification[],
			): ns is FoldableNotification[] {
				return represent.type === "renote" || represent.type === "reaction";
			}
			if (!check(ns)) {
				return represent;
			}
			return {
				...represent,
				folded: true,
				userIds: ns.map((nn) => nn.userId),
				users: ns.map((nn) => nn.user),
				notifications: ns!,
				id: `G-${lastId}-${key}`,
			} as NotificationFolded;
		},
	);
}
