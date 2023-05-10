-- This file should undo anything in `up.sql`
ALTER TABLE posts ALTER COLUMN posted_at DROP NOT NULL;