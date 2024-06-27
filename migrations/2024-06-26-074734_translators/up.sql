-- Your SQL goes here
CREATE TYPE t_type AS ENUM ('editor', 'typer', 'cleaner')
;

CREATE TABLE
	"translators" (
		"id" UUID NOT NULL PRIMARY KEY,
		-- 
		"type" "t_type" ARRAY NOT NULL
	)
;