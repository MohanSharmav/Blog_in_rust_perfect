-- Add migration script here
ALTER TABLE categories ADD COLUMN created_at TIMESTAMP;
ALTER TABLE categories ALTER COLUMN created_at SET DEFAULT now();