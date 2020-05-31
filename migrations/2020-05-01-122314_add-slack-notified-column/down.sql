-- This file should undo anything in `up.sql`
BEGIN TRANSACTION;

-- 既存のテーブルをリネーム
ALTER TABLE papers RENAME TO papers_temp;
-- 新しいテーブルを作成（元々のテーブル名と同じ名前で）
CREATE TABLE papers
(
    id          INTEGER  NOT NULL PRIMARY KEY AUTOINCREMENT,
    title       TEXT     NOT NULL,
    url         TEXT     NOT NULL UNIQUE,
    pdf_url     TEXT     NOT NULL UNIQUE,
    category_id INTEGER  NOT NULL,
    summary     TEXT     NOT NULL,
    comment     TEXT     NOT NULL,
    accepted    INTEGER  NOT NULL,
    updated     DATETIME NOT NULL,
    published   DATETIME NOT NULL,
    created     DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (category_id) REFERENCES categories (id)
);
-- レコードを全て移す
INSERT INTO papers(id, title, url, pdf_url, category_id, summary, comment, accepted, updated, published, created)
 SELECT (id, title, url, pdf_url, category_id, summary, comment, accepted, updated, published, created) FROM papers_temp;
-- 元のテーブルを削除
DROP TABLE papers_temp;

COMMIT;
