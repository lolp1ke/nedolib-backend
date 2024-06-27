-- Your SQL goes here
CREATE TABLE
	"titles_teams" (
		"title_id" UUID NOT NULL REFERENCES "titles" ("id") ON DELETE CASCADE,
		"team_id" UUID NOT NULL REFERENCES "teams" ("id") ON DELETE CASCADE,
		PRIMARY KEY ("title_id", "team_id")
	)
;