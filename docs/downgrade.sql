BEGIN;

DELETE FROM "migrations" WHERE name IN (
    'AddMastodonSubscriptionType1715181461692',
    'SwSubscriptionAccessToken1709395223611',
    'UserProfileMentions1711075007936',
    'ClientCredentials1713108561474',
    'TurnOffCatLanguage1720107645050',
    'RefactorScheduledPosts1716804636187',
    'RemoveEnumTypenameSuffix1716462794927',
    'CreateScheduledNote1714728200194',
    'AddBackTimezone1715351290096',
    'UserprofileJsonbToArray1714270605574',
    'DropUnusedUserprofileColumns1714259023878',
    'AntennaJsonbToArray1714192520471',
    'AddUserProfileLanguage1714888400293',
    'DropUnusedIndexes1714643926317',
    'AlterAkaType1714099399879',
    'AddDriveFileUsage1713451569342',
    'ConvertCwVarcharToText1713225866247',
    'FixChatFileConstraint1712855579316',
    'DropTimeZone1712425488543',
    'ExpandNoteEdit1711936358554',
    'markLocalFilesNsfwByDefault1709305200000',
    'FixMutingIndices1710690239308',
    'NoteFile1710304584214',
    'RenameMetaColumns1705944717480',
    'SeparateHardMuteWordsAndPatterns1706413792769',
    'IndexAltTextAndCw1708872574733',
    'Pgroonga1698420787202',
    'ChangeDefaultConfigs1709251460718',
    'AddReplyMuting1704851359889',
    'FixNoteUrlIndex1709129810501',
    'RemoveCharts1709047957489',
    'DropUserProfileLanguage1708452631156',
    'EmojiModerator1692825433698',
    'RemoveNsfwDetection1705848938166',
    'FirefishUrlMove1707850084123',
    'RemoveNativeUtilsMigration1705877093218'
);

-- addMastodonSubscriptionType
ALTER TABLE "sw_subscription" DROP COLUMN "subscriptionTypes";
DROP TYPE "push_subscription_type";

-- sw-subscription-per-access-token
ALTER TABLE "sw_subscription" DROP CONSTRAINT "FK_98a1aa2db2a5253924f42f38767";
ALTER TABLE "sw_subscription" DROP COLUMN "appAccessTokenId";

-- user-profile-mentions
ALTER TABLE "user_profile" DROP COLUMN "mentions";

-- client-credential-support
ALTER TABLE "access_token" ALTER COLUMN "userId" SET NOT NULL;

-- turn-off-cat-language
ALTER TABLE "user" DROP COLUMN "readCatLanguage";

-- refactor-scheduled-post
CREATE TABLE "scheduled_note" (
	"id" character varying(32) NOT NULL PRIMARY KEY,
	"noteId" character varying(32) NOT NULL,
	"userId" character varying(32) NOT NULL,
	"scheduledAt" TIMESTAMP WITH TIME ZONE NOT NULL
);
COMMENT ON COLUMN "scheduled_note"."noteId" IS 'The ID of the temporarily created note that corresponds to the schedule.';
CREATE EXTENSION pgcrypto;
CREATE FUNCTION generate_scheduled_note_id(size int) RETURNS text AS $$	DECLARE
	characters text := 'abcdefghijklmnopqrstuvwxyz0123456789';
	bytes bytea := gen_random_bytes(size);
	l int := length(characters);
	i int := 0;
	output text := '';
	BEGIN
		WHILE i < size LOOP
			output := output || substr(characters, get_byte(bytes, i) % l + 1, 1);
			i := i + 1;
		END LOOP;
		RETURN output;
	END;
$$ LANGUAGE plpgsql VOLATILE;
INSERT INTO "scheduled_note" ("id", "noteId", "userId", "scheduledAt") (SELECT generate_scheduled_note_id(16), "id", "userId", "scheduledAt" FROM "note" WHERE "note"."scheduledAt" IS NOT NULL);
DROP EXTENSION pgcrypto;
DROP FUNCTION "generate_scheduled_note_id";
CREATE INDEX "IDX_noteId_ScheduledNote" ON "scheduled_note" ("noteId");
CREATE INDEX "IDX_userId_ScheduledNote" ON "scheduled_note" ("userId");
ALTER TABLE "scheduled_note" ADD FOREIGN KEY ("noteId") REFERENCES "note"("id") ON DELETE CASCADE ON UPDATE NO ACTION;
ALTER TABLE "scheduled_note" ADD FOREIGN KEY ("userId") REFERENCES "user"("id") ON DELETE CASCADE ON UPDATE NO ACTION;
ALTER TABLE "note" DROP COLUMN "scheduledAt";

-- remove-enum-typename-suffix
ALTER TYPE "antenna_src" RENAME TO "antenna_src_enum";
ALTER TYPE "drive_file_usage_hint" RENAME TO "drive_file_usage_hint_enum";
ALTER TYPE "muted_note_reason" RENAME TO "muted_note_reason_enum";
ALTER TYPE "note_visibility" RENAME TO "note_visibility_enum";
ALTER TYPE "notification_type" RENAME TO "notification_type_enum";
ALTER TYPE "page_visibility" RENAME TO "page_visibility_enum";
ALTER TYPE "poll_note_visibility" RENAME TO "poll_notevisibility_enum";
ALTER TYPE "relay_status" RENAME TO "relay_status_enum";
ALTER TYPE "user_emoji_mod_perm" RENAME TO "user_emojimodperm_enum";
ALTER TYPE "user_profile_ffvisibility" RENAME TO "user_profile_ffvisibility_enum";
ALTER TYPE "user_profile_muting_notification_types" RENAME TO "user_profile_mutingnotificationtypes_enum";

-- create-scheduled-note
DROP TABLE "scheduled_note";

-- userprofile-jsonb-to-array
ALTER TABLE "user_profile" RENAME COLUMN "mutedInstances" TO "mutedInstances_old";
ALTER TABLE "user_profile" ADD COLUMN "mutedInstances" jsonb NOT NULL DEFAULT '[]';
UPDATE "user_profile" SET "mutedInstances" = to_jsonb("mutedInstances_old");
ALTER TABLE "user_profile" DROP COLUMN "mutedInstances_old";
ALTER TABLE "user_profile" RENAME COLUMN "mutedWords" TO "mutedWords_old";
ALTER TABLE "user_profile" ADD COLUMN "mutedWords" jsonb NOT NULL DEFAULT '[]';
CREATE TEMP TABLE "BCrsGgLCUeMMLARy" ("userId" character varying(32), "kws" jsonb NOT NULL DEFAULT '[]');
INSERT INTO "BCrsGgLCUeMMLARy" ("userId", "kws") SELECT "userId", jsonb_agg("X"."w") FROM (SELECT "userId", to_jsonb(string_to_array(unnest("mutedWords_old"), ' ')) AS "w" FROM "user_profile") AS "X" GROUP BY "userId";
UPDATE "user_profile" SET "mutedWords" = "kws" FROM "BCrsGgLCUeMMLARy" WHERE "user_profile"."userId" = "BCrsGgLCUeMMLARy"."userId";
ALTER TABLE "user_profile" DROP COLUMN "mutedWords_old";

-- drop-unused-userprofile-columns
ALTER TABLE "user_profile" ADD "room" jsonb NOT NULL DEFAULT '{}';
COMMENT ON COLUMN "user_profile"."room" IS 'The room data of the User.';
ALTER TABLE "user_profile" ADD "clientData" jsonb NOT NULL DEFAULT '{}';
COMMENT ON COLUMN "user_profile"."clientData" IS 'The client-specific data of the User.';

-- antenna-jsonb-to-array
UPDATE "antenna" SET "instances" = '{""}' WHERE "instances" = '{}';
ALTER TABLE "antenna" RENAME COLUMN "instances" TO "instances_old";
ALTER TABLE "antenna" ADD COLUMN "instances" jsonb NOT NULL DEFAULT '[]';
UPDATE "antenna" SET "instances" = to_jsonb("instances_old");
ALTER TABLE "antenna" DROP COLUMN "instances_old";
UPDATE "antenna" SET "keywords" = '{""}' WHERE "keywords" = '{}';
ALTER TABLE "antenna" RENAME COLUMN "keywords" TO "keywords_old";
ALTER TABLE "antenna" ADD COLUMN "keywords" jsonb NOT NULL DEFAULT '[]';
CREATE TEMP TABLE "QvPNcMitBFkqqBgm" ("id" character varying(32), "kws" jsonb NOT NULL DEFAULT '[]');
INSERT INTO "QvPNcMitBFkqqBgm" ("id", "kws") SELECT "id", jsonb_agg("X"."w") FROM (SELECT "id", to_jsonb(string_to_array(unnest("keywords_old"), ' ')) AS "w" FROM "antenna") AS "X" GROUP BY "id";
UPDATE "antenna" SET "keywords" = "kws" FROM "QvPNcMitBFkqqBgm" WHERE "antenna"."id" = "QvPNcMitBFkqqBgm"."id";
ALTER TABLE "antenna" DROP COLUMN "keywords_old";
UPDATE "antenna" SET "excludeKeywords" = '{""}' WHERE "excludeKeywords" = '{}';
ALTER TABLE "antenna" RENAME COLUMN "excludeKeywords" TO "excludeKeywords_old";
ALTER TABLE "antenna" ADD COLUMN "excludeKeywords" jsonb NOT NULL DEFAULT '[]';
CREATE TEMP TABLE "MZvVSjHzYcGXmGmz" ("id" character varying(32), "kws" jsonb NOT NULL DEFAULT '[]');
INSERT INTO "MZvVSjHzYcGXmGmz" ("id", "kws") SELECT "id", jsonb_agg("X"."w") FROM (SELECT "id", to_jsonb(string_to_array(unnest("excludeKeywords_old"), ' ')) AS "w" FROM "antenna") AS "X" GROUP BY "id";
UPDATE "antenna" SET "excludeKeywords" = "kws" FROM "MZvVSjHzYcGXmGmz" WHERE "antenna"."id" = "MZvVSjHzYcGXmGmz"."id";
ALTER TABLE "antenna" DROP COLUMN "excludeKeywords_old";

-- drop-unused-indexes
CREATE INDEX "IDX_01f4581f114e0ebd2bbb876f0b" ON "note_reaction" ("createdAt");
CREATE INDEX "IDX_0610ebcfcfb4a18441a9bcdab2" ON "poll" ("userId");
CREATE INDEX "IDX_25dfc71b0369b003a4cd434d0b" ON "note" ("attachedFileTypes");
CREATE INDEX "IDX_2710a55f826ee236ea1a62698f" ON "hashtag" ("mentionedUsersCount");
CREATE INDEX "IDX_4c02d38a976c3ae132228c6fce" ON "hashtag" ("mentionedRemoteUsersCount");
CREATE INDEX "IDX_51c063b6a133a9cb87145450f5" ON "note" ("fileIds");
CREATE INDEX "IDX_54ebcb6d27222913b908d56fd8" ON "note" ("mentions");
CREATE INDEX "IDX_7fa20a12319c7f6dc3aed98c0a" ON "poll" ("userHost");
CREATE INDEX "IDX_88937d94d7443d9a99a76fa5c0" ON "note" ("tags");
CREATE INDEX "IDX_b11a5e627c41d4dc3170f1d370" ON "notification" ("createdAt");
CREATE INDEX "IDX_c8dfad3b72196dd1d6b5db168a" ON "drive_file" ("createdAt");
CREATE INDEX "IDX_d57f9030cd3af7f63ffb1c267c" ON "hashtag" ("attachedUsersCount");
CREATE INDEX "IDX_e5848eac4940934e23dbc17581" ON "drive_file" ("uri");
CREATE INDEX "IDX_fa99d777623947a5b05f394cae" ON "user" ("tags");

-- alter-aka-type
ALTER TABLE "user" RENAME COLUMN "alsoKnownAs" TO "alsoKnownAsOld";
ALTER TABLE "user" ADD COLUMN "alsoKnownAs" text;
UPDATE "user" SET "alsoKnownAs" = array_to_string("alsoKnownAsOld", ',');
COMMENT ON COLUMN "user"."alsoKnownAs" IS 'URIs the user is known as too';
ALTER TABLE "user" DROP COLUMN "alsoKnownAsOld";

-- AddDriveFileUsage
ALTER TABLE "drive_file" DROP COLUMN "usageHint";
DROP TYPE "drive_file_usage_hint_enum";

-- convert-cw-varchar-to-text
DROP INDEX "IDX_8e3bbbeb3df04d1a8105da4c8f";
ALTER TABLE "note" ALTER COLUMN "cw" TYPE character varying(512);
CREATE INDEX "IDX_8e3bbbeb3df04d1a8105da4c8f" ON "note" USING "pgroonga" ("cw" pgroonga_varchar_full_text_search_ops_v2);

-- fix-chat-file-constraint
ALTER TABLE "messaging_message" DROP CONSTRAINT "FK_535def119223ac05ad3fa9ef64b";
ALTER TABLE "messaging_message" ADD CONSTRAINT "FK_535def119223ac05ad3fa9ef64b" FOREIGN KEY ("fileId") REFERENCES "drive_file"("id") ON DELETE CASCADE ON UPDATE NO ACTION;

-- expand-note-edit
ALTER TABLE "note_edit" DROP COLUMN "emojis";

-- markLocalFilesNsfwByDefault
ALTER TABLE "meta" DROP COLUMN "markLocalFilesNsfwByDefault";

-- fix-muting-indices
DROP INDEX "IDX_renote_muting_createdAt";
DROP INDEX "IDX_renote_muting_muteeId";
DROP INDEX "IDX_renote_muting_muterId";
DROP INDEX "IDX_reply_muting_createdAt";
DROP INDEX "IDX_reply_muting_muteeId";
DROP INDEX "IDX_reply_muting_muterId";
CREATE INDEX "IDX_renote_muting_createdAt" ON "muting" ("createdAt");
CREATE INDEX "IDX_renote_muting_muteeId" ON "muting" ("muteeId");
CREATE INDEX "IDX_renote_muting_muterId" ON "muting" ("muterId");

-- note-file
DROP TABLE "note_file";

-- rename-meta-columns
ALTER TABLE "meta" RENAME COLUMN "tosUrl" TO "ToSUrl";
ALTER TABLE "meta" RENAME COLUMN "objectStorageUseSsl" TO "objectStorageUseSSL";
ALTER TABLE "meta" RENAME COLUMN "customMotd" TO "customMOTD";

-- separate-hard-mute-words-and-patterns
UPDATE "user_profile" SET "mutedWords" = "mutedWords" || array_to_json("mutedPatterns")::jsonb;
ALTER TABLE "user_profile" DROP "mutedPatterns";

-- index-alt-text-and-cw
DROP INDEX "IDX_f4f7b93d05958527300d79ac82";
DROP INDEX "IDX_8e3bbbeb3df04d1a8105da4c8f";

-- pgroonga
DROP INDEX "IDX_f27f5d88941e57442be75ba9c8";
DROP INDEX "IDX_065d4d8f3b5adb4a08841eae3c";
DROP INDEX "IDX_fcb770976ff8240af5799e3ffc";
DROP EXTENSION pgroonga CASCADE;

-- change-default-configs
ALTER TABLE "user_profile" ALTER COLUMN "noCrawle" SET DEFAULT false;
ALTER TABLE "user_profile" ALTER COLUMN "publicReactions" SET DEFAULT false;
ALTER TABLE "meta" ALTER COLUMN "disableRegistration" SET DEFAULT false;

-- reply-muting
DROP TABLE "reply_muting";

-- remove-charts
CREATE TABLE public.__chart__ap_request (
    id integer NOT NULL,
    date integer NOT NULL,
    "___deliverFailed" integer DEFAULT 0 NOT NULL,
    "___deliverSucceeded" integer DEFAULT 0 NOT NULL,
    "___inboxReceived" integer DEFAULT 0 NOT NULL
);
CREATE SEQUENCE public.__chart__ap_request_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;
ALTER SEQUENCE public.__chart__ap_request_id_seq OWNED BY public.__chart__ap_request.id;

CREATE TABLE public.__chart__drive (
    id integer NOT NULL,
    date integer NOT NULL,
    "___local_incCount" integer DEFAULT '0'::bigint NOT NULL,
    "___local_incSize" integer DEFAULT '0'::bigint NOT NULL,
    "___local_decCount" integer DEFAULT '0'::bigint NOT NULL,
    "___local_decSize" integer DEFAULT '0'::bigint NOT NULL,
    "___remote_incCount" integer DEFAULT '0'::bigint NOT NULL,
    "___remote_incSize" integer DEFAULT '0'::bigint NOT NULL,
    "___remote_decCount" integer DEFAULT '0'::bigint NOT NULL,
    "___remote_decSize" integer DEFAULT '0'::bigint NOT NULL
);
CREATE SEQUENCE public.__chart__drive_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;
ALTER SEQUENCE public.__chart__drive_id_seq OWNED BY public.__chart__drive.id;

CREATE TABLE public.__chart__federation (
    id integer NOT NULL,
    date integer NOT NULL,
    "unique_temp___deliveredInstances" character varying[] DEFAULT '{}'::character varying[] NOT NULL,
    "___deliveredInstances" smallint DEFAULT '0'::smallint NOT NULL,
    "unique_temp___inboxInstances" character varying[] DEFAULT '{}'::character varying[] NOT NULL,
    "___inboxInstances" smallint DEFAULT '0'::smallint NOT NULL,
    unique_temp___stalled character varying[] DEFAULT '{}'::character varying[] NOT NULL,
    ___stalled smallint DEFAULT '0'::smallint NOT NULL,
    ___sub smallint DEFAULT '0'::smallint NOT NULL,
    ___pub smallint DEFAULT '0'::smallint NOT NULL,
    ___pubsub smallint DEFAULT '0'::smallint NOT NULL,
    "___subActive" smallint DEFAULT '0'::smallint NOT NULL,
    "___pubActive" smallint DEFAULT '0'::smallint NOT NULL
);
CREATE SEQUENCE public.__chart__federation_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;
ALTER SEQUENCE public.__chart__federation_id_seq OWNED BY public.__chart__federation.id;

CREATE TABLE public.__chart__hashtag (
    id integer NOT NULL,
    date integer NOT NULL,
    "group" character varying(128) NOT NULL,
    ___local_users integer DEFAULT 0 NOT NULL,
    ___remote_users integer DEFAULT 0 NOT NULL,
    unique_temp___local_users character varying[] DEFAULT '{}'::character varying[] NOT NULL,
    unique_temp___remote_users character varying[] DEFAULT '{}'::character varying[] NOT NULL
);
CREATE SEQUENCE public.__chart__hashtag_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;
ALTER SEQUENCE public.__chart__hashtag_id_seq OWNED BY public.__chart__hashtag.id;

CREATE TABLE public.__chart__instance (
    id integer NOT NULL,
    date integer NOT NULL,
    "group" character varying(128) NOT NULL,
    ___requests_failed smallint DEFAULT '0'::bigint NOT NULL,
    ___requests_succeeded smallint DEFAULT '0'::bigint NOT NULL,
    ___requests_received smallint DEFAULT '0'::bigint NOT NULL,
    ___notes_total integer DEFAULT '0'::bigint NOT NULL,
    ___notes_inc integer DEFAULT '0'::bigint NOT NULL,
    ___notes_dec integer DEFAULT '0'::bigint NOT NULL,
    ___notes_diffs_normal integer DEFAULT '0'::bigint NOT NULL,
    ___notes_diffs_reply integer DEFAULT '0'::bigint NOT NULL,
    ___notes_diffs_renote integer DEFAULT '0'::bigint NOT NULL,
    ___users_total integer DEFAULT '0'::bigint NOT NULL,
    ___users_inc smallint DEFAULT '0'::bigint NOT NULL,
    ___users_dec smallint DEFAULT '0'::bigint NOT NULL,
    ___following_total integer DEFAULT '0'::bigint NOT NULL,
    ___following_inc smallint DEFAULT '0'::bigint NOT NULL,
    ___following_dec smallint DEFAULT '0'::bigint NOT NULL,
    ___followers_total integer DEFAULT '0'::bigint NOT NULL,
    ___followers_inc smallint DEFAULT '0'::bigint NOT NULL,
    ___followers_dec smallint DEFAULT '0'::bigint NOT NULL,
    "___drive_totalFiles" integer DEFAULT '0'::bigint NOT NULL,
    "___drive_incFiles" integer DEFAULT '0'::bigint NOT NULL,
    "___drive_incUsage" integer DEFAULT '0'::bigint NOT NULL,
    "___drive_decFiles" integer DEFAULT '0'::bigint NOT NULL,
    "___drive_decUsage" integer DEFAULT '0'::bigint NOT NULL,
    "___notes_diffs_withFile" integer DEFAULT 0 NOT NULL
);
CREATE SEQUENCE public.__chart__instance_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;
ALTER SEQUENCE public.__chart__instance_id_seq OWNED BY public.__chart__instance.id;

CREATE TABLE public.__chart__network (
    id integer NOT NULL,
    date integer NOT NULL,
    "___incomingRequests" integer DEFAULT '0'::bigint NOT NULL,
    "___outgoingRequests" integer DEFAULT '0'::bigint NOT NULL,
    "___totalTime" integer DEFAULT '0'::bigint NOT NULL,
    "___incomingBytes" integer DEFAULT '0'::bigint NOT NULL,
    "___outgoingBytes" integer DEFAULT '0'::bigint NOT NULL
);
CREATE SEQUENCE public.__chart__network_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;
ALTER SEQUENCE public.__chart__network_id_seq OWNED BY public.__chart__network.id;

CREATE TABLE public.__chart__notes (
    id integer NOT NULL,
    date integer NOT NULL,
    ___local_total integer DEFAULT '0'::bigint NOT NULL,
    ___local_inc integer DEFAULT '0'::bigint NOT NULL,
    ___local_dec integer DEFAULT '0'::bigint NOT NULL,
    ___local_diffs_normal integer DEFAULT '0'::bigint NOT NULL,
    ___local_diffs_reply integer DEFAULT '0'::bigint NOT NULL,
    ___local_diffs_renote integer DEFAULT '0'::bigint NOT NULL,
    ___remote_total integer DEFAULT '0'::bigint NOT NULL,
    ___remote_inc integer DEFAULT '0'::bigint NOT NULL,
    ___remote_dec integer DEFAULT '0'::bigint NOT NULL,
    ___remote_diffs_normal integer DEFAULT '0'::bigint NOT NULL,
    ___remote_diffs_reply integer DEFAULT '0'::bigint NOT NULL,
    ___remote_diffs_renote integer DEFAULT '0'::bigint NOT NULL,
    "___local_diffs_withFile" integer DEFAULT 0 NOT NULL,
    "___remote_diffs_withFile" integer DEFAULT 0 NOT NULL
);
CREATE SEQUENCE public.__chart__notes_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;
ALTER SEQUENCE public.__chart__notes_id_seq OWNED BY public.__chart__notes.id;

CREATE TABLE public.__chart__per_user_drive (
    id integer NOT NULL,
    date integer NOT NULL,
    "group" character varying(128) NOT NULL,
    "___totalCount" integer DEFAULT '0'::bigint NOT NULL,
    "___totalSize" integer DEFAULT '0'::bigint NOT NULL,
    "___incCount" smallint DEFAULT '0'::bigint NOT NULL,
    "___incSize" integer DEFAULT '0'::bigint NOT NULL,
    "___decCount" smallint DEFAULT '0'::bigint NOT NULL,
    "___decSize" integer DEFAULT '0'::bigint NOT NULL
);
CREATE SEQUENCE public.__chart__per_user_drive_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;
ALTER SEQUENCE public.__chart__per_user_drive_id_seq OWNED BY public.__chart__per_user_drive.id;

CREATE TABLE public.__chart__per_user_following (
    id integer NOT NULL,
    date integer NOT NULL,
    "group" character varying(128) NOT NULL,
    ___local_followings_total integer DEFAULT '0'::bigint NOT NULL,
    ___local_followings_inc smallint DEFAULT '0'::bigint NOT NULL,
    ___local_followings_dec smallint DEFAULT '0'::bigint NOT NULL,
    ___local_followers_total integer DEFAULT '0'::bigint NOT NULL,
    ___local_followers_inc smallint DEFAULT '0'::bigint NOT NULL,
    ___local_followers_dec smallint DEFAULT '0'::bigint NOT NULL,
    ___remote_followings_total integer DEFAULT '0'::bigint NOT NULL,
    ___remote_followings_inc smallint DEFAULT '0'::bigint NOT NULL,
    ___remote_followings_dec smallint DEFAULT '0'::bigint NOT NULL,
    ___remote_followers_total integer DEFAULT '0'::bigint NOT NULL,
    ___remote_followers_inc smallint DEFAULT '0'::bigint NOT NULL,
    ___remote_followers_dec smallint DEFAULT '0'::bigint NOT NULL
);
CREATE SEQUENCE public.__chart__per_user_following_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;
ALTER SEQUENCE public.__chart__per_user_following_id_seq OWNED BY public.__chart__per_user_following.id;

CREATE TABLE public.__chart__per_user_notes (
    id integer NOT NULL,
    date integer NOT NULL,
    "group" character varying(128) NOT NULL,
    ___total integer DEFAULT '0'::bigint NOT NULL,
    ___inc smallint DEFAULT '0'::bigint NOT NULL,
    ___dec smallint DEFAULT '0'::bigint NOT NULL,
    ___diffs_normal smallint DEFAULT '0'::bigint NOT NULL,
    ___diffs_reply smallint DEFAULT '0'::bigint NOT NULL,
    ___diffs_renote smallint DEFAULT '0'::bigint NOT NULL,
    "___diffs_withFile" smallint DEFAULT '0'::smallint NOT NULL
);
CREATE SEQUENCE public.__chart__per_user_notes_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;
ALTER SEQUENCE public.__chart__per_user_notes_id_seq OWNED BY public.__chart__per_user_notes.id;

CREATE TABLE public.__chart__per_user_reaction (
    id integer NOT NULL,
    date integer NOT NULL,
    "group" character varying(128) NOT NULL,
    ___local_count smallint DEFAULT '0'::bigint NOT NULL,
    ___remote_count smallint DEFAULT '0'::bigint NOT NULL
);
CREATE SEQUENCE public.__chart__per_user_reaction_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;
ALTER SEQUENCE public.__chart__per_user_reaction_id_seq OWNED BY public.__chart__per_user_reaction.id;

CREATE TABLE public.__chart__test (
    id integer NOT NULL,
    date integer NOT NULL,
    "group" character varying(128),
    ___foo_total bigint NOT NULL,
    ___foo_inc bigint NOT NULL,
    ___foo_dec bigint NOT NULL
);
CREATE TABLE public.__chart__test_grouped (
    id integer NOT NULL,
    date integer NOT NULL,
    "group" character varying(128),
    ___foo_total bigint NOT NULL,
    ___foo_inc bigint NOT NULL,
    ___foo_dec bigint NOT NULL
);
CREATE SEQUENCE public.__chart__test_grouped_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;
ALTER SEQUENCE public.__chart__test_grouped_id_seq OWNED BY public.__chart__test_grouped.id;

CREATE SEQUENCE public.__chart__test_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;
ALTER SEQUENCE public.__chart__test_id_seq OWNED BY public.__chart__test.id;

CREATE TABLE public.__chart__test_unique (
    id integer NOT NULL,
    date integer NOT NULL,
    "group" character varying(128),
    ___foo character varying[] DEFAULT '{}'::character varying[] NOT NULL
);
CREATE SEQUENCE public.__chart__test_unique_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;
ALTER SEQUENCE public.__chart__test_unique_id_seq OWNED BY public.__chart__test_unique.id;

CREATE TABLE public.__chart__users (
    id integer NOT NULL,
    date integer NOT NULL,
    ___local_total integer DEFAULT '0'::bigint NOT NULL,
    ___local_inc smallint DEFAULT '0'::bigint NOT NULL,
    ___local_dec smallint DEFAULT '0'::bigint NOT NULL,
    ___remote_total integer DEFAULT '0'::bigint NOT NULL,
    ___remote_inc smallint DEFAULT '0'::bigint NOT NULL,
    ___remote_dec smallint DEFAULT '0'::bigint NOT NULL
);
CREATE SEQUENCE public.__chart__users_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;
ALTER SEQUENCE public.__chart__users_id_seq OWNED BY public.__chart__users.id;

CREATE TABLE public.__chart_day__ap_request (
    id integer NOT NULL,
    date integer NOT NULL,
    "___deliverFailed" integer DEFAULT 0 NOT NULL,
    "___deliverSucceeded" integer DEFAULT 0 NOT NULL,
    "___inboxReceived" integer DEFAULT 0 NOT NULL
);
CREATE SEQUENCE public.__chart_day__ap_request_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;
ALTER SEQUENCE public.__chart_day__ap_request_id_seq OWNED BY public.__chart_day__ap_request.id;

CREATE TABLE public.__chart_day__drive (
    id integer NOT NULL,
    date integer NOT NULL,
    "___local_incCount" integer DEFAULT '0'::bigint NOT NULL,
    "___local_incSize" integer DEFAULT '0'::bigint NOT NULL,
    "___local_decCount" integer DEFAULT '0'::bigint NOT NULL,
    "___local_decSize" integer DEFAULT '0'::bigint NOT NULL,
    "___remote_incCount" integer DEFAULT '0'::bigint NOT NULL,
    "___remote_incSize" integer DEFAULT '0'::bigint NOT NULL,
    "___remote_decCount" integer DEFAULT '0'::bigint NOT NULL,
    "___remote_decSize" integer DEFAULT '0'::bigint NOT NULL
);
CREATE SEQUENCE public.__chart_day__drive_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;
ALTER SEQUENCE public.__chart_day__drive_id_seq OWNED BY public.__chart_day__drive.id;

CREATE TABLE public.__chart_day__federation (
    id integer NOT NULL,
    date integer NOT NULL,
    "unique_temp___deliveredInstances" character varying[] DEFAULT '{}'::character varying[] NOT NULL,
    "___deliveredInstances" smallint DEFAULT '0'::smallint NOT NULL,
    "unique_temp___inboxInstances" character varying[] DEFAULT '{}'::character varying[] NOT NULL,
    "___inboxInstances" smallint DEFAULT '0'::smallint NOT NULL,
    unique_temp___stalled character varying[] DEFAULT '{}'::character varying[] NOT NULL,
    ___stalled smallint DEFAULT '0'::smallint NOT NULL,
    ___sub smallint DEFAULT '0'::smallint NOT NULL,
    ___pub smallint DEFAULT '0'::smallint NOT NULL,
    ___pubsub smallint DEFAULT '0'::smallint NOT NULL,
    "___subActive" smallint DEFAULT '0'::smallint NOT NULL,
    "___pubActive" smallint DEFAULT '0'::smallint NOT NULL
);
CREATE SEQUENCE public.__chart_day__federation_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;
ALTER SEQUENCE public.__chart_day__federation_id_seq OWNED BY public.__chart_day__federation.id;

CREATE TABLE public.__chart_day__hashtag (
    id integer NOT NULL,
    date integer NOT NULL,
    "group" character varying(128) NOT NULL,
    ___local_users integer DEFAULT 0 NOT NULL,
    ___remote_users integer DEFAULT 0 NOT NULL,
    unique_temp___local_users character varying[] DEFAULT '{}'::character varying[] NOT NULL,
    unique_temp___remote_users character varying[] DEFAULT '{}'::character varying[] NOT NULL
);
CREATE SEQUENCE public.__chart_day__hashtag_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;
ALTER SEQUENCE public.__chart_day__hashtag_id_seq OWNED BY public.__chart_day__hashtag.id;

CREATE TABLE public.__chart_day__instance (
    id integer NOT NULL,
    date integer NOT NULL,
    "group" character varying(128) NOT NULL,
    ___requests_failed smallint DEFAULT '0'::bigint NOT NULL,
    ___requests_succeeded smallint DEFAULT '0'::bigint NOT NULL,
    ___requests_received smallint DEFAULT '0'::bigint NOT NULL,
    ___notes_total integer DEFAULT '0'::bigint NOT NULL,
    ___notes_inc integer DEFAULT '0'::bigint NOT NULL,
    ___notes_dec integer DEFAULT '0'::bigint NOT NULL,
    ___notes_diffs_normal integer DEFAULT '0'::bigint NOT NULL,
    ___notes_diffs_reply integer DEFAULT '0'::bigint NOT NULL,
    ___notes_diffs_renote integer DEFAULT '0'::bigint NOT NULL,
    ___users_total integer DEFAULT '0'::bigint NOT NULL,
    ___users_inc smallint DEFAULT '0'::bigint NOT NULL,
    ___users_dec smallint DEFAULT '0'::bigint NOT NULL,
    ___following_total integer DEFAULT '0'::bigint NOT NULL,
    ___following_inc smallint DEFAULT '0'::bigint NOT NULL,
    ___following_dec smallint DEFAULT '0'::bigint NOT NULL,
    ___followers_total integer DEFAULT '0'::bigint NOT NULL,
    ___followers_inc smallint DEFAULT '0'::bigint NOT NULL,
    ___followers_dec smallint DEFAULT '0'::bigint NOT NULL,
    "___drive_totalFiles" integer DEFAULT '0'::bigint NOT NULL,
    "___drive_incFiles" integer DEFAULT '0'::bigint NOT NULL,
    "___drive_incUsage" integer DEFAULT '0'::bigint NOT NULL,
    "___drive_decFiles" integer DEFAULT '0'::bigint NOT NULL,
    "___drive_decUsage" integer DEFAULT '0'::bigint NOT NULL,
    "___notes_diffs_withFile" integer DEFAULT 0 NOT NULL
);
CREATE SEQUENCE public.__chart_day__instance_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;
ALTER SEQUENCE public.__chart_day__instance_id_seq OWNED BY public.__chart_day__instance.id;

CREATE TABLE public.__chart_day__network (
    id integer NOT NULL,
    date integer NOT NULL,
    "___incomingRequests" integer DEFAULT '0'::bigint NOT NULL,
    "___outgoingRequests" integer DEFAULT '0'::bigint NOT NULL,
    "___totalTime" integer DEFAULT '0'::bigint NOT NULL,
    "___incomingBytes" integer DEFAULT '0'::bigint NOT NULL,
    "___outgoingBytes" integer DEFAULT '0'::bigint NOT NULL
);
CREATE SEQUENCE public.__chart_day__network_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;
ALTER SEQUENCE public.__chart_day__network_id_seq OWNED BY public.__chart_day__network.id;

CREATE TABLE public.__chart_day__notes (
    id integer NOT NULL,
    date integer NOT NULL,
    ___local_total integer DEFAULT '0'::bigint NOT NULL,
    ___local_inc integer DEFAULT '0'::bigint NOT NULL,
    ___local_dec integer DEFAULT '0'::bigint NOT NULL,
    ___local_diffs_normal integer DEFAULT '0'::bigint NOT NULL,
    ___local_diffs_reply integer DEFAULT '0'::bigint NOT NULL,
    ___local_diffs_renote integer DEFAULT '0'::bigint NOT NULL,
    ___remote_total integer DEFAULT '0'::bigint NOT NULL,
    ___remote_inc integer DEFAULT '0'::bigint NOT NULL,
    ___remote_dec integer DEFAULT '0'::bigint NOT NULL,
    ___remote_diffs_normal integer DEFAULT '0'::bigint NOT NULL,
    ___remote_diffs_reply integer DEFAULT '0'::bigint NOT NULL,
    ___remote_diffs_renote integer DEFAULT '0'::bigint NOT NULL,
    "___local_diffs_withFile" integer DEFAULT 0 NOT NULL,
    "___remote_diffs_withFile" integer DEFAULT 0 NOT NULL
);
CREATE SEQUENCE public.__chart_day__notes_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;
ALTER SEQUENCE public.__chart_day__notes_id_seq OWNED BY public.__chart_day__notes.id;

CREATE TABLE public.__chart_day__per_user_drive (
    id integer NOT NULL,
    date integer NOT NULL,
    "group" character varying(128) NOT NULL,
    "___totalCount" integer DEFAULT '0'::bigint NOT NULL,
    "___totalSize" integer DEFAULT '0'::bigint NOT NULL,
    "___incCount" smallint DEFAULT '0'::bigint NOT NULL,
    "___incSize" integer DEFAULT '0'::bigint NOT NULL,
    "___decCount" smallint DEFAULT '0'::bigint NOT NULL,
    "___decSize" integer DEFAULT '0'::bigint NOT NULL
);
CREATE SEQUENCE public.__chart_day__per_user_drive_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;
ALTER SEQUENCE public.__chart_day__per_user_drive_id_seq OWNED BY public.__chart_day__per_user_drive.id;

CREATE TABLE public.__chart_day__per_user_following (
    id integer NOT NULL,
    date integer NOT NULL,
    "group" character varying(128) NOT NULL,
    ___local_followings_total integer DEFAULT '0'::bigint NOT NULL,
    ___local_followings_inc smallint DEFAULT '0'::bigint NOT NULL,
    ___local_followings_dec smallint DEFAULT '0'::bigint NOT NULL,
    ___local_followers_total integer DEFAULT '0'::bigint NOT NULL,
    ___local_followers_inc smallint DEFAULT '0'::bigint NOT NULL,
    ___local_followers_dec smallint DEFAULT '0'::bigint NOT NULL,
    ___remote_followings_total integer DEFAULT '0'::bigint NOT NULL,
    ___remote_followings_inc smallint DEFAULT '0'::bigint NOT NULL,
    ___remote_followings_dec smallint DEFAULT '0'::bigint NOT NULL,
    ___remote_followers_total integer DEFAULT '0'::bigint NOT NULL,
    ___remote_followers_inc smallint DEFAULT '0'::bigint NOT NULL,
    ___remote_followers_dec smallint DEFAULT '0'::bigint NOT NULL
);
CREATE SEQUENCE public.__chart_day__per_user_following_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;
ALTER SEQUENCE public.__chart_day__per_user_following_id_seq OWNED BY public.__chart_day__per_user_following.id;

CREATE TABLE public.__chart_day__per_user_notes (
    id integer NOT NULL,
    date integer NOT NULL,
    "group" character varying(128) NOT NULL,
    ___total integer DEFAULT '0'::bigint NOT NULL,
    ___inc smallint DEFAULT '0'::bigint NOT NULL,
    ___dec smallint DEFAULT '0'::bigint NOT NULL,
    ___diffs_normal smallint DEFAULT '0'::bigint NOT NULL,
    ___diffs_reply smallint DEFAULT '0'::bigint NOT NULL,
    ___diffs_renote smallint DEFAULT '0'::bigint NOT NULL,
    "___diffs_withFile" smallint DEFAULT '0'::smallint NOT NULL
);
CREATE SEQUENCE public.__chart_day__per_user_notes_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;
ALTER SEQUENCE public.__chart_day__per_user_notes_id_seq OWNED BY public.__chart_day__per_user_notes.id;

CREATE TABLE public.__chart_day__per_user_reaction (
    id integer NOT NULL,
    date integer NOT NULL,
    "group" character varying(128) NOT NULL,
    ___local_count smallint DEFAULT '0'::bigint NOT NULL,
    ___remote_count smallint DEFAULT '0'::bigint NOT NULL
);
CREATE SEQUENCE public.__chart_day__per_user_reaction_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;
ALTER SEQUENCE public.__chart_day__per_user_reaction_id_seq OWNED BY public.__chart_day__per_user_reaction.id;

CREATE TABLE public.__chart_day__users (
    id integer NOT NULL,
    date integer NOT NULL,
    ___local_total integer DEFAULT '0'::bigint NOT NULL,
    ___local_inc smallint DEFAULT '0'::bigint NOT NULL,
    ___local_dec smallint DEFAULT '0'::bigint NOT NULL,
    ___remote_total integer DEFAULT '0'::bigint NOT NULL,
    ___remote_inc smallint DEFAULT '0'::bigint NOT NULL,
    ___remote_dec smallint DEFAULT '0'::bigint NOT NULL
);
CREATE SEQUENCE public.__chart_day__users_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;
ALTER SEQUENCE public.__chart_day__users_id_seq OWNED BY public.__chart_day__users.id;

-- emoji-moderator
ALTER TABLE "user" DROP COLUMN "emojiModPerm";
DROP TYPE "public"."user_emojimodperm_enum";

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
