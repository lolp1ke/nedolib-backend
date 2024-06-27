-- Your SQL goes here
CREATE TABLE
	"chapters" (
		"id" UUID NOT NULL PRIMARY key,
		-- 
		"volume_id" UUID NOT NULL REFERENCES "volumes" ("id"),
		-- 
		"number" FLOAT NOT NULL,
		-- 
		"craeted" TIMESTAMP NOT NULL DEFAULT now (),
		"altered" TIMESTAMP DEFAULT NULL
	)
;