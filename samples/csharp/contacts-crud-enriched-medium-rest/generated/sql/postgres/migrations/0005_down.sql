-- Rollback migration for postgres
ALTER TABLE addresses DROP COLUMN "latitude";
GO
ALTER TABLE addresses DROP COLUMN "label_note";
GO
ALTER TABLE addresses DROP COLUMN "floor";
GO
ALTER TABLE addresses DROP COLUMN "score";
GO
ALTER TABLE addresses DROP COLUMN "external_id";
GO
ALTER TABLE addresses DROP COLUMN "is_verified";
GO
ALTER TABLE addresses DROP COLUMN "verified_at";
GO
ALTER TABLE addresses DROP COLUMN "stamp";
