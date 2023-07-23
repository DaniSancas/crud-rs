-- Add migration script here
CREATE TABLE IF NOT EXISTS articles
(
    `id` INTEGER PRIMARY KEY NOT NULL,
    `title` VARCHAR(250)     NOT NULL,
    `content` TEXT           NOT NULL
);

INSERT INTO articles (title, content) VALUES ("Example article", "This is the first article.");
