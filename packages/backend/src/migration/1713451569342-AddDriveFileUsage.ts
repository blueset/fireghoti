import type { MigrationInterface, QueryRunner } from "typeorm";

export class AddDriveFileUsage1713451569342 implements MigrationInterface {
	public async up(queryRunner: QueryRunner): Promise<void> {
		await queryRunner.query(
			`ALTER TABLE "drive_file" ADD "usageHint" character varying(16) DEFAULT NULL`,
		);
	}

	public async down(queryRunner: QueryRunner): Promise<void> {
		await queryRunner.query(
			`ALTER TABLE "drive_file" DROP COLUMN "usageHint"`
		);
	}
}
