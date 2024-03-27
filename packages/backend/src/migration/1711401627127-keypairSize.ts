import { MigrationInterface, QueryRunner } from "typeorm";

export class KeypairSize1711401627127 implements MigrationInterface {
	name = "KeypairSize1711401627127";

	public async up(queryRunner: QueryRunner): Promise<void> {
		await queryRunner.query(
			`ALTER TABLE "user_keypair" ALTER COLUMN "privateKey" TYPE character varying(8192)`,
		);
		await queryRunner.query(
			`ALTER TABLE "user_keypair" ALTER COLUMN "publicKey" TYPE character varying(8192)`,
		);
	}

	public async down(queryRunner: QueryRunner): Promise<void> {
		await queryRunner.query(
			`ALTER TABLE "user_keypair" ALTER COLUMN "privateKey" TYPE character varying(4096)`,
		);
		await queryRunner.query(
			`ALTER TABLE "user_keypair" ALTER COLUMN "publicKey" TYPE character varying(4096)`,
		);
	}
}
