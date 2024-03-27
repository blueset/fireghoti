import type Bull from "bull";

import { UserProfiles } from "@/models/index.js";
import { Not, And } from "typeorm";
import { queueLogger } from "../../logger.js";
import { verifyLink } from "@/services/fetch-rel-me.js";
import { inspect } from "node:util";

const logger = queueLogger.createSubLogger("verify-links");

export async function verifyLinks(
	job: Bull.Job<Record<string, unknown>>,
	done: any,
): Promise<void> {
	logger.info("Verifying links...");

	const usersToVerify = (await UserProfiles.find({
		where: {
			fields: Not([]),
			...(job.data?.all ? {} : {userHost: ""}),
		},
		relations: ["user"],
	})).filter(user => user.fields?.length);
	for (const user of usersToVerify) {
		for (const field of user.fields) {
			if (!field || field.name === "" || field.value === "") {
				continue;
			}
			if (field.value.startsWith("http") && (user.user?.username || user.url)) {
				field.verified = await verifyLink(field.value, user.user?.username, user.url);
			}
		}
		if (user.fields.length > 0) {
			try {
				await UserProfiles.update(user.userId, {
					fields: user.fields,
				});
			} catch (e) {
				logger.error(`Failed to update user ${user.userId}:\n${inspect(e)}`);
				done(e);
			}
		}
	}

	logger.succ(`All ${usersToVerify.length} links successfully verified.`);
	done();
}
