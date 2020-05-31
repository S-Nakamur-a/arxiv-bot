-- This file should undo anything in `up.sql`-- Your SQL goes here
ALTER TABLE papers ADD slack_notified BOOLEAN NOT NULL DEFAULT FALSE;
