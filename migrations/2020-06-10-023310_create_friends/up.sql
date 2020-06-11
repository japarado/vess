-- Your SQL goes here
CREATE TABLE friends(
	user_id INTEGER REFERENCES users(id) ON DELETE CASCADE,
	friend_id INTEGER REFERENCES users(id) ON DELETE CASCADE,
	PRIMARY KEY(user_id, friend_id)
)
