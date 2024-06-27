-- This file should undo anything in `up.sql`
ALTER TABLE "translators"
DROP IF EXISTS "team_id"
;

ALTER TABLE "translators"
DROP IF EXISTS "t_type"
;