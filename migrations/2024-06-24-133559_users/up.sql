-- Your SQL goes here
CREATE TABLE
	"users" (
		"id" UUID NOT NULL PRIMARY KEY,
		-- 
		"username" TEXT NOT NULL UNIQUE,
		"hash" TEXT NOT NULL,
		-- 
		"email" TEXT NOT NULL UNIQUE,
		-- 
		"created" TIMESTAMP NOT NULL DEFAULT now (),
		"altered" TIMESTAMP DEFAULT NULL
	)
;