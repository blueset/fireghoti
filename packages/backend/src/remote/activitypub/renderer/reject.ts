import { config } from "@/config.js";
import type { User } from "@/models/entities/user.js";

export default (object: any, user: { id: User["id"] }) => ({
	type: "Reject",
	actor: `${config.url}/users/${user.id}`,
	object,
});
