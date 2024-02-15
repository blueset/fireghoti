BEGIN;

DELETE FROM "migrations" WHERE name IN (
    'RemoveNsfwDetection1705848938166',
    'FirefishUrlMove1707850084123',
    'RemoveNativeUtilsMigration1705877093218'
);

-- remove-nsfw-detection
ALTER TABLE "user_profile" ADD "autoSensitive" boolean NOT NULL DEFAULT false;
ALTER TABLE "meta" ADD "enableSensitiveMediaDetectionForVideos" boolean NOT NULL DEFAULT false;
ALTER TABLE "meta" ADD "setSensitiveFlagAutomatically" boolean NOT NULL DEFAULT false;
CREATE TYPE "public"."meta_sensitivemediadetectionsensitivity_enum" AS ENUM('medium', 'low', 'high', 'veryLow', 'veryHigh');
ALTER TABLE "meta" ADD "sensitiveMediaDetectionSensitivity" "public"."meta_sensitivemediadetectionsensitivity_enum" NOT NULL DEFAULT 'medium';
CREATE TYPE "public"."meta_sensitivemediadetection_enum" AS ENUM('none', 'all', 'local', 'remote');
ALTER TABLE "meta" ADD "sensitiveMediaDetection" "public"."meta_sensitivemediadetection_enum" NOT NULL DEFAULT 'none';
ALTER TABLE "drive_file" ADD "maybePorn" boolean NOT NULL DEFAULT false;
ALTER TABLE "drive_file" ADD "maybeSensitive" boolean NOT NULL DEFAULT false;
COMMENT ON COLUMN "drive_file"."maybeSensitive" IS 'Whether the DriveFile is NSFW. (predict)';

-- firefish-url-move
UPDATE "meta" SET "repositoryUrl" = 'https://git.joinfirefish.org/firefish/firefish';
UPDATE "meta" SET "feedbackUrl" = 'https://git.joinfirefish.org/firefish/firefish/issues';

-- remove-native-utils-migration
CREATE TABLE "seaql_migrations" (
    version character varying NOT NULL,
    applied_at bigint NOT NULL
);
INSERT INTO "seaql_migrations" (version, applied_at)
VALUES
    ('m20230531_180824_drop_reversi', 1705876632),
    ('m20230627_185451_index_note_url', 1705876632),
    ('m20230709_000510_move_antenna_to_cache', 1705876632),
    ('m20230806_170616_fix_antenna_stream_ids', 1705876632),
    ('m20230904_013244_is_indexable', 1705876632),
    ('m20231002_143323_remove_integrations', 1705876632)
;

COMMIT;
