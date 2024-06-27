-- Your SQL goes here
CREATE TABLE
	"sessions" (
		"id" UUID NOT NULL PRIMARY KEY,
		-- 
		"user_id" UUID NOT NULL REFERENCES "users" ("id"),
		-- 
		"ip" TEXT NOT NULL,
		--
		"created" TIMESTAMP NOT NULL DEFAULT now (),
		"altered" TIMESTAMP DEFAULT NULL
	)
;