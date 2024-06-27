-- Your SQL goes here
CREATE TABLE
	"volumes" (
		"id" UUID NOT NULL PRIMARY key,
		-- 
		"title_id" UUID NOT NULL REFERENCES "titles" ("id"),
		-- 
		"number" FLOAT NOT NULL,
		-- 
		"craeted" TIMESTAMP NOT NULL DEFAULT now (),
		"altered" TIMESTAMP DEFAULT NULL
	)
;