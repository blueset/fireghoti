import type { MigrationInterface, QueryRunner } from "typeorm";

import { v4 as uuid } from "uuid";
import { genRsaKeyPair } from "@/misc/gen-key-pair.js";
import { User } from "@/models/entities/user.js";
import { UserProfile } from "@/models/entities/user-profile.js";
import { Users } from "@/models/index.js";
import { IsNull } from "typeorm";
import { generateUserToken, genIdAt, hashPassword } from "backend-rs";
import { UserKeypair } from "@/models/entities/user-keypair.js";
import { UsedUsername } from "@/models/entities/used-username.js";
import { db } from "@/db/postgre.js";

async function createSystemUser(username: string) {
	const password = uuid();

	// Generate hash of password
	const hash = hashPassword(password);

	// Generate secret
	const secret = generateUserToken();

	const keyPair = await genRsaKeyPair(4096);

	const exists = await Users.existsBy({
		usernameLower: username.toLowerCase(),
		host: IsNull(),
	});

	if (exists) {
		return;
	}

	const now = new Date();

	// Start transaction
	await db.transaction(async (transactionalEntityManager) => {
		const account = await transactionalEntityManager
			.insert(User, {
				id: genIdAt(now),
				createdAt: now,
				username: username,
				usernameLower: username.toLowerCase(),
				host: null,
				token: secret,
				isAdmin: false,
				isLocked: true,
				isExplorable: false,
				isBot: true,
			})
			.then((x) =>
				transactionalEntityManager.findOneByOrFail(User, x.identifiers[0]),
			);

		await transactionalEntityManager.insert(UserKeypair, {
			publicKey: keyPair.publicKey,
			privateKey: keyPair.privateKey,
			userId: account.id,
		});

		await transactionalEntityManager.insert(UserProfile, {
			userId: account.id,
			autoAcceptFollowed: false,
			password: hash,
		});

		await transactionalEntityManager.insert(UsedUsername, {
			createdAt: now,
			username: username.toLowerCase(),
		});
	});
}

export class CreateSystemActors1720618854585 implements MigrationInterface {
	public async up(_: QueryRunner): Promise<void> {
		if (!db.isInitialized) {
			db.initialize();
		}
		await createSystemUser("instance.actor");
		await createSystemUser("relay.actor");
	}

	public async down(_: QueryRunner): Promise<void> {
		/* You don't need to revert this migration. */
	}
}
