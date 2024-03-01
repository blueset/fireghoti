export class ChangeDefaultConfigs1709251460718 {
	name = "ChangeDefaultConfigs1709251460718";

	async up(queryRunner) {
		await queryRunner.query(
			`ALTER TABLE "meta" ALTER COLUMN "disableRegistration" SET DEFAULT true`,
		);
		await queryRunner.query(
			`ALTER TABLE "user_profile" ALTER COLUMN "publicReactions" SET DEFAULT true`,
		);
		await queryRunner.query(
			`ALTER TABLE "user_profile" ALTER COLUMN "noCrawle" SET DEFAULT true`,
		);
	}

	async down(queryRunner) {
		await queryRunner.query(
			`ALTER TABLE "user_profile" ALTER COLUMN "noCrawle" SET DEFAULT false`,
		);
		await queryRunner.query(
			`ALTER TABLE "user_profile" ALTER COLUMN "publicReactions" SET DEFAULT false`,
		);
		await queryRunner.query(
			`ALTER TABLE "meta" ALTER COLUMN "disableRegistration" SET DEFAULT false`,
		);
	}
}
