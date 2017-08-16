-- Your SQL goes here
CREATE TABLE events (
	id SERIAL PRIMARY KEY,
	title VARCHAR NOT NULL,
	location VARCHAR,
	start_time TIMESTAMP NOT NULL,
	end_time TIMESTAMP 
);
