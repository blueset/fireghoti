import { MigrationInterface, QueryRunner } from "typeorm";

export class RemoveMentionedUsersColumn1710688552234
	implements MigrationInterface
{
	public async up(queryRunner: QueryRunner): Promise<void> {
		await queryRunner.query(
			`ALTER TABLE "note" DROP COLUMN "mentionedRemoteUsers"`,
		);
	}

	public async down(queryRunner: QueryRunner): Promise<void> {
		await queryRunner.query(
			`ALTER TABLE "note" ADD "mentionedRemoteUsers" TEXT NOT NULL DEFAULT '[]'::text`,
		);
		await queryRunner.query(`CREATE TEMP TABLE IF NOT EXISTS "temp_mentions" AS
			SELECT "id", "url", "uri", "username", "host"
			FROM "user"
			JOIN "user_profile" ON "user"."id" = "user_profile". "userId" WHERE "user"."host" IS NOT NULL`);
		await queryRunner.query(
			`CREATE UNIQUE INDEX "temp_mentions_id" ON "temp_mentions"("id")`,
		);
		await queryRunner.query(`UPDATE "note" SET "mentionedRemoteUsers" = (
			SELECT COALESCE(json_agg(row_to_json("data")::jsonb - 'id')::text, '[]') FROM "temp_mentions" AS "data"
			WHERE "data"."id" = ANY("note"."mentions")
			)`);
	}
}
