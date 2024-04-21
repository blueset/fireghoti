import { config } from "@/config.js";
import type { ILocalUser } from "@/models/entities/user.js";

export default (user: ILocalUser, target: any, object: any) => ({
	type: "Add",
	actor: `${config.url}/users/${user.id}`,
	target,
	object,
});
