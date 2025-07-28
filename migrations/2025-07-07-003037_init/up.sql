CREATE TABLE channels (
    id                      TEXT    NOT NULL PRIMARY KEY UNIQUE,
    name                    TEXT    NOT NULL,
    url                     TEXT    NOT NULL,
    is_subscribed           BOOLEAN NOT NULL,
    subscribers_count       BIGINT  NOT NULL,

    added_at                BIGINT  NOT NULL
);

CREATE TABLE videos (
    id                      TEXT    NOT NULL PRIMARY KEY UNIQUE,
    channel_id              TEXT    NOT NULL REFERENCES channels(id) ON DELETE CASCADE,
    url                     TEXT    NOT NULL,
    title                   TEXT    NOT NULL,
    description             TEXT    NOT NULL,
    watch_counter           BIGINT  NOT NULL,
    duration_seconds        BIGINT  NOT NULL,
    likes_count             BIGINT  NOT NULL,
    view_count              BIGINT  NOT NULL,
    comments_count          BIGINT  NOT NULL,
    published_at            BIGINT  NOT NULL,

    added_at                BIGINT  NOT NULL
);

CREATE TABLE tags (
    id                      TEXT    NOT NULL PRIMARY KEY,
    name                    TEXT    NOT NULL UNIQUE,

    added_at                BIGINT  NOT NULL
);

-- Junction table
CREATE TABLE video_tags (
    video_id                TEXT    NOT NULL REFERENCES videos(id) ON DELETE CASCADE,
    tag_id                  TEXT    NOT NULL REFERENCES tags(id) ON DELETE CASCADE,

    PRIMARY KEY (video_id, tag_id)
);

CREATE TABLE watch_history (
    id                      TEXT    NOT NULL PRIMARY KEY,
    video_id                TEXT    NOT NULL REFERENCES videos(id) ON DELETE CASCADE,
    channel_id              TEXT    NOT NULL REFERENCES channels(id) ON DELETE CASCADE,
    watch_duration_seconds  BIGINT  NOT NULL,
    session_start_date      BIGINT  NOT NULL,
    session_end_date        BIGINT  NOT NULL,

    added_at                BIGINT  NOT NULL
);

CREATE TRIGGER increment_watch_counter
AFTER INSERT ON watch_history
FOR EACH ROW
BEGIN
    UPDATE videos
    SET watch_counter = watch_counter + 1
    WHERE id = NEW.video_id;
END;
