-- Your SQL goes here
CREATE TABLE people (
	id INTEGER NOT NULL PRIMARY KEY,
	username VARCHAR NOT NULL,
	role VARCHAR NOT NULL,
	state INTEGER NOT NULL DEFAULT 0,
	roomid INTEGER
)
