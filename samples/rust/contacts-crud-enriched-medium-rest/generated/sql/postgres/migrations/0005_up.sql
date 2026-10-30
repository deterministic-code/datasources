-- Generated migration for postgres
ALTER TABLE addresses ADD COLUMN "latitude" INTEGER;
GO
ALTER TABLE addresses ADD COLUMN "label_note" VARCHAR(128);
GO
ALTER TABLE addresses ADD COLUMN "floor" BIGINT;
GO
ALTER TABLE addresses ADD COLUMN "score" NUMERIC(10, 2);
GO
ALTER TABLE addresses ADD COLUMN "external_id" UUID;
GO
ALTER TABLE addresses ADD COLUMN "is_verified" BOOLEAN;
GO
ALTER TABLE addresses ADD COLUMN "verified_at" TIMESTAMPTZ;
GO
ALTER TABLE addresses ADD COLUMN "stamp" BYTEA;
