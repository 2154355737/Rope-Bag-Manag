-- Create likes tables if not exist
CREATE TABLE IF NOT EXISTS package_likes (
  user_id INTEGER NOT NULL,
  package_id INTEGER NOT NULL,
  created_at TEXT NOT NULL DEFAULT (CURRENT_TIMESTAMP),
  PRIMARY KEY (user_id, package_id)
);
CREATE INDEX IF NOT EXISTS idx_package_likes_user ON package_likes(user_id);
CREATE INDEX IF NOT EXISTS idx_package_likes_pkg ON package_likes(package_id);

CREATE TABLE IF NOT EXISTS post_likes (
  user_id INTEGER NOT NULL,
  post_id INTEGER NOT NULL,
  created_at TEXT NOT NULL DEFAULT (CURRENT_TIMESTAMP),
  PRIMARY KEY (user_id, post_id)
);
CREATE INDEX IF NOT EXISTS idx_post_likes_user ON post_likes(user_id);
CREATE INDEX IF NOT EXISTS idx_post_likes_post ON post_likes(post_id); 