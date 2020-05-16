-- Your SQL goes here
CREATE TABLE comments(
	id SERIAL PRIMARY KEY,
	contents TEXT NOT NULL,
	user_id INTEGER REFERENCES users(id) NOT NULL,
	post_id INTEGER REFERENCES posts(id) NOT NULL
)
