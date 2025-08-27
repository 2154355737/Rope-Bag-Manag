-- Add review fields to posts table
-- SQLite doesn't support IF NOT EXISTS for ADD COLUMN in older versions,
-- so duplicate column errors should be ignored by the caller.
ALTER TABLE posts ADD COLUMN review_status TEXT DEFAULT 'pending';
ALTER TABLE posts ADD COLUMN review_comment TEXT;
ALTER TABLE posts ADD COLUMN reviewer_id INTEGER;
ALTER TABLE posts ADD COLUMN reviewed_at DATETIME;

-- Helpful indexes
CREATE INDEX IF NOT EXISTS idx_posts_review_status ON posts(review_status); 