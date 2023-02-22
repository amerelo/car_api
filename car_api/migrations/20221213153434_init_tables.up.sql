-- Add up migration script here

CREATE TABLE users (
	id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
	email VARCHAR UNIQUE NOT NULL,
	user_name VARCHAR NOT NULL,
	password_hash VARCHAR NOT NULL,
	created_at TIMESTAMP NOT NULL DEFAULT current_timestamp,
	updated_at TIMESTAMP  NOT NULL DEFAULT current_timestamp
);

-- car details and bank details

CREATE TABLE car (
	id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
	user_id uuid NOT NULL REFERENCES users (id) ON DELETE CASCADE,
	plate VARCHAR NOT NULL,
	model VARCHAR NOT NULL,
	created_at TIMESTAMP NOT NULL DEFAULT current_timestamp,
	updated_at TIMESTAMP  NOT NULL DEFAULT current_timestamp
);

CREATE TABLE bank_details (
	id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
	user_id uuid NOT NULL REFERENCES users (id) ON DELETE CASCADE,
	country VARCHAR NOT NULL,
	iban VARCHAR NOT NULL,
	account_holder VARCHAR NOT NULL,
	created_at TIMESTAMP NOT NULL DEFAULT current_timestamp,
	updated_at TIMESTAMP  NOT NULL DEFAULT current_timestamp
);

