export class FixNoteUrlIndex1709129810501 {
	name = "FixNoteUrlIndex1709129810501";

	async up(queryRunner) {
		await queryRunner.query(`DROP INDEX "IDX_note_url"`);
		await queryRunner.query(`CREATE INDEX "IDX_note_url" ON "note" ("url")`);
	}
	async down(queryRunner) {
		/* You don't revert this migration */
	}
}
