-- Your SQL goes here
PRAGMA foreign_keys = true;

CREATE TABLE authors
(
    id   INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    name TEXT    NOT NULL UNIQUE
);

CREATE TABLE categories
(
    id   INTEGER     NOT NULL PRIMARY KEY AUTOINCREMENT,
    name TEXT UNIQUE NOT NULL
);

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

CREATE INDEX arxiv_papers_category_id ON papers (category_id);

CREATE TABLE paper_authors
(
    paper_id  INTEGER NOT NULL,
    author_id INTEGER NOT NULL,
    PRIMARY KEY (paper_id, author_id),
    FOREIGN KEY (paper_id) REFERENCES papers (id),
    FOREIGN KEY (author_id) REFERENCES authors (id)
);
