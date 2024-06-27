-- Your SQL goes here
CREATE TABLE
	"reviews" (
		"id" UUID NOT NULL PRIMARY key,
		-- 
		"profile_id" UUID NOT NULL REFERENCES "profiles" ("id"),
		"title_id" UUID NOT NULL REFERENCES "titles" ("id"),
		--
		"content" TEXT NOT NULL,
		-- 
		"created" TIMESTAMP NOT NULL DEFAULT now (),
		"altered" TIMESTAMP DEFAULT NULL
	)
;