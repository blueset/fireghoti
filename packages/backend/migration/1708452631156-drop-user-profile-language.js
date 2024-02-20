export class DropUserProfileLanguage1708452631156 {
	name = "DropUserProfileLanguage1708452631156";

	async up(queryRunner) {
		await queryRunner.query(`ALTER TABLE "user_profile" DROP COLUMN "lang"`);
	}

	async down(queryRunner) {
		await queryRunner.query(
			`ALTER TABLE "user_profile" ADD COLUMN "lang" character varying(32)`,
		);
	}
}
