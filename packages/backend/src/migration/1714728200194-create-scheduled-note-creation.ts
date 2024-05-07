import type { MigrationInterface, QueryRunner } from "typeorm";

export class CreateScheduledNoteCreation1714728200194
	implements MigrationInterface
{
	public async up(queryRunner: QueryRunner): Promise<void> {
		await queryRunner.query(
			`CREATE TABLE "scheduled_note_creation" (
				"id" character varying(32) NOT NULL PRIMARY KEY,
				"noteId" character varying(32) NOT NULL,
				"userId" character varying(32) NOT NULL,
				"scheduledAt" TIMESTAMP WITHOUT TIME ZONE NOT NULL
		)`,
		);
		await queryRunner.query(`
			COMMENT ON COLUMN "scheduled_note_creation"."noteId" IS 'The ID of note scheduled.'
		`);
		await queryRunner.query(`
			CREATE INDEX "IDX_noteId_ScheduledNoteCreation" ON "scheduled_note_creation" ("noteId")
		`);
		await queryRunner.query(`
			CREATE INDEX "IDX_userId_ScheduledNoteCreation" ON "scheduled_note_creation" ("userId")
		`);
		await queryRunner.query(`
			ALTER TABLE "scheduled_note_creation"
			ADD FOREIGN KEY ("noteId") REFERENCES "note"("id")
			ON DELETE CASCADE
			ON UPDATE NO ACTION
		`);
		await queryRunner.query(`
			ALTER TABLE "scheduled_note_creation"
			ADD FOREIGN KEY ("userId") REFERENCES "user"("id")
			ON DELETE CASCADE
			ON UPDATE NO ACTION
		`);
	}

	public async down(queryRunner: QueryRunner): Promise<void> {
		await queryRunner.query(`DROP TABLE "scheduled_note_creation"`);
	}
}
