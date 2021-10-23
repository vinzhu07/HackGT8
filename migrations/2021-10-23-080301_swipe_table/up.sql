-- Your SQL goes here
CREATE TABLE swipes (
  id INTEGER NOT NULL PRIMARY KEY,
  cloth_id INTEGER NOT NULL,
  love_status BOOLEAN NOT NULL CHECK (love_status IN (0, 1))
);