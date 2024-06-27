-- Your SQL goes here
CREATE type "status" AS ENUM ('on_going', 'finished', 'abandoned')
;

CREATE type "tag" AS ENUM ('school')
;

CREATE type "genre" AS ENUM ('issekai')
;

CREATE TABLE
	"titles" (
		"id" UUID NOT NULL PRIMARY KEY,
		-- 
		"author_id" UUID NOT NULL REFERENCES "authors" ("id") ON DELETE CASCADE,
		"artist_id" UUID NOT NULL REFERENCES "artists" ("id") ON DELETE CASCADE,
		-- 
		"name" TEXT NOT NULL,
		"alternative_names" TEXT ARRAY NOT NULL,
		"release" INT NOT NULL,
		"status" "status" NOT NULL,
		-- 
		"tags" "tag" ARRAY NOT NULL,
		"genres" "genre" ARRAY NOT NULL,
		-- 
		"created" TIMESTAMP NOT NULL DEFAULT now (),
		"altered" TIMESTAMP DEFAULT NULL
	)
;