-- This file should undo anything in `up.sql`
ALTER TABLE users RENAME is_active TO active;
