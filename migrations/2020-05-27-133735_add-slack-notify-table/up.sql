-- Your SQL goes here
CREATE TABLE slack_notifications
(
    id          INTEGER     NOT NULL PRIMARY KEY AUTOINCREMENT,
    paper_id    INTEGER     NOT NULL,
    slack_url   TEXT        NOT NULL,
    updated_at  DATETIME    NOT NULL,
    send        BOOLEAN     NOT NULL DEFAULT FALSE,
    created     DATETIME    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (paper_id) REFERENCES papers (id)
);

CREATE INDEX not_yet_slack_notifications ON slack_notifications(slack_url, send);
CREATE UNIQUE INDEX slack_notification_paper ON slack_notifications(slack_url, paper_id, updated_at);
