-- Your SQL goes here
CREATE TABLE
	"profiles" (
		"id" UUID NOT NULL PRIMARY KEY,
		-- 
		"user_id" UUID UNIQUE REFERENCES "users" ("id") ON DELETE CASCADE,
		"translator_id" UUID UNIQUE REFERENCES "translators" ("id") ON DELETE CASCADE,
		"admin_id" UUID UNIQUE REFERENCES "admins" ("id") ON DELETE CASCADE,
		-- 
		"name" TEXT,
		"bio" TEXT,
		--
		"profile_picture_path" TEXT,
		"banner_picture_path" TEXT
	)
;