CREATE TABLE IF NOT EXISTS application
(
    id                  INTEGER PRIMARY KEY NOT NULL,
    name                TEXT                NOT NULL,
    date                TEXT                NOT NULL,
    description         TEXT                NOT NULL DEFAULT '',
    resume_sent         BOOLEAN             NOT NULL DEFAULT 0,
    coverletter_sent    BOOLEAN             NOT NULL DEFAULT 0,
    response_date       TEXT                         DEFAULT NULL,
    interview_date      TEXT                         DEFAULT NULL
);
