-- Your SQL goes here
CREATE TABLE clothes (
  id INTEGER NOT NULL PRIMARY KEY,
  gender VARCHAR NOT NULL,
  master_category TEXT NOT NULL,
  sub_category TEXT NOT NULL,
  article_type TEXT NOT NULL,
  base_color TEXT NOT NULL,
  season TEXT NOT NULL,
  usage TEXT NOT NULL,
  product_display_name TEXT NOT NULL
);