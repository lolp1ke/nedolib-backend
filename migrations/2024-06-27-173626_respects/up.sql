-- Your SQL goes here
CREATE TABLE
	"respects" (
		"id" UUID NOT NULL PRIMARY key,
		--
		"profile_id" UUID NOT NULL REFERENCES "profiles" ("id"),
		"comment_id" UUID REFERENCES "comments" ("id"),
		"review_id" UUID REFERENCES "reviews" ("id"),
		-- 
		"isPositive" BOOLEAN NOT NULL,
		-- 
		"created" TIMESTAMP NOT NULL DEFAULT now (),
		"altered" TIMESTAMP DEFAULT NULL
	)
;