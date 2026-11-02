-- Generated migration for sqlite
ALTER TABLE addresses ADD COLUMN "latitude" INTEGER;
GO
ALTER TABLE addresses ADD COLUMN "label_note" VARCHAR(128);
GO
ALTER TABLE addresses ADD COLUMN "floor" INTEGER;
GO
ALTER TABLE addresses ADD COLUMN "score" NUMERIC;
GO
ALTER TABLE addresses ADD COLUMN "external_id" TEXT;
GO
ALTER TABLE addresses ADD COLUMN "is_verified" BOOLEAN;
GO
ALTER TABLE addresses ADD COLUMN "verified_at" TEXT;
GO
ALTER TABLE addresses ADD COLUMN "stamp" BLOB;
