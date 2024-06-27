-- Your SQL goes here
CREATE TABLE
	"comments" (
		"id" UUID NOT NULL PRIMARY key,
		-- 
		"user_id" UUID NOT NULL REFERENCES "profiles" ("id"),
		"page_id" UUID NOT NULL REFERENCES "pages" ("id"),
		-- 
		"content" TEXT NOT NULL,
		-- 
		"created" TIMESTAMP NOT NULL DEFAULT now (),
		"altered" TIMESTAMP DEFAULT NULL
	)
;