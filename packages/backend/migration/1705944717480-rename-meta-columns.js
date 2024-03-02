export class RenameMetaColumns1705944717480 {
	name = "RenameMetaColumns1705944717480";

	async up(queryRunner) {
		await queryRunner.query(
			`ALTER TABLE "meta" RENAME COLUMN "customMOTD" TO "customMotd"`,
		);
		await queryRunner.query(
			`ALTER TABLE "meta" RENAME COLUMN "objectStorageUseSSL" TO "objectStorageUseSsl"`,
		);
		await queryRunner.query(
			`ALTER TABLE "meta" RENAME COLUMN "ToSUrl" TO "tosUrl"`,
		);
	}

	async down(queryRunner) {
		await queryRunner.query(
			`ALTER TABLE "meta" RENAME COLUMN "tosUrl" TO "ToSUrl"`,
		);
		await queryRunner.query(
			`ALTER TABLE "meta" RENAME COLUMN "objectStorageUseSsl" TO "objectStorageUseSSL"`,
		);
		await queryRunner.query(
			`ALTER TABLE "meta" RENAME COLUMN "customMotd" TO "customMOTD"`,
		);
	}
}
