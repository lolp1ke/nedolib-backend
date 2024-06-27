-- Your SQL goes here
ALTER TABLE "translators" ADD "team_id" UUID NOT NULL UNIQUE REFERENCES "teams" ("id") ON DELETE CASCADE
;

ALTER TABLE "translators" ADD "t_type" "t_type" ARRAY NOT NULL
;