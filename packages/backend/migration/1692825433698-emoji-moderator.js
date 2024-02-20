export class EmojiModerator1692825433698 {
	name = "EmojiModerator1692825433698";

	async up(queryRunner) {
		await queryRunner.query(
			`CREATE TYPE "public"."user_emojimodperm_enum" AS ENUM('unauthorized', 'add', 'mod', 'full')`,
		);
		await queryRunner.query(
			`ALTER TABLE "user" ADD "emojiModPerm" "public"."user_emojimodperm_enum" NOT NULL DEFAULT 'unauthorized'`,
		);
	}

	async down(queryRunner) {
		await queryRunner.query(`ALTER TABLE "user" DROP COLUMN "emojiModPerm"`);
		await queryRunner.query(`DROP TYPE "public"."user_emojimodperm_enum"`);
	}
}
