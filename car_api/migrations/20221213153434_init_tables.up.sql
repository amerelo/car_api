-- Add up migration script here

CREATE TABLE users (
	user_id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
	email VARCHAR UNIQUE NOT NULL,
	password_hash VARCHAR NOT NULL,
	created_at timestamptz NOT NULL DEFAULT now(),
	updated_at TIMESTAMP  NOT NULL DEFAULT current_timestamp
);

-- car details and bank details

CREATE TABLE car (
	car_id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
	user_id uuid NOT NULL REFERENCES users (user_id) ON DELETE CASCADE,
	car_plate VARCHAR NOT NULL
	-- region ?
	-- car type ?
	-- created_at timestamptz NOT NULL DEFAULT now(),
	-- updated_at TIMESTAMP  NOT NULL DEFAULT current_timestamp
);

-- CREATE INDEX car_user ON car (note_id, user_id);

-- CREATE TABLE bank_details (
-- 	car_id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
-- 	user_id uuid NOT NULL REFERENCES users (user_id) ON DELETE CASCADE,
-- 	car_plate VARCHAR NOT NULL,
-- 	-- region ?
-- 	-- car type ?
-- 	-- created_at timestamptz NOT NULL DEFAULT now(),
-- 	-- updated_at TIMESTAMP  NOT NULL DEFAULT current_timestamp
-- );

-- CREATE TABLE note (
-- 	note_id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
-- 	user_id uuid NOT NULL REFERENCES users (user_id) ON DELETE CASCADE,
-- 	name VARCHAR NOT NULL,
-- 	content VARCHAR NOT NULL DEFAULT '',
-- 	tags VARCHAR [] NOT NULL DEFAULT array[]::varchar[],
-- 	created_at timestamptz NOT NULL DEFAULT now(),
-- 	updated_at TIMESTAMP  NOT NULL DEFAULT current_timestamp
-- );

-- CREATE INDEX note_user_group ON note (note_id, user_id);

