-- Your SQL goes here
CREATE TABLE
	"pages" (
		"id" UUID NOT NULL PRIMARY key,
		-- 
		"chapter_id" UUID NOT NULL REFERENCES "chapters" ("id"),
		-- 
		"number" INT NOT NULL,
		"picture_path" TEXT NOT NULL
	)
;