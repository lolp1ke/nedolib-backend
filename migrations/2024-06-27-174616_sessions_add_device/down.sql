-- This file should undo anything in `up.sql`
ALTER TABLE "sessions"
DROP IF EXISTS "device"
;