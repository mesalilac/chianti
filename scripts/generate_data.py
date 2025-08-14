import sqlite3
import sys
import os
import time
import argparse
from random import randrange
from faker import Faker
from rich import print
from dataclasses import dataclass


TEN_YEARS = 10 * 365 * 24 * 60 * 60

MIN_VIDEO_PUBLISHED_AT = int(1e4)
MAX_VIDEO_PUBLISHED_AT = TEN_YEARS
MIN_SESSION_START = int(1e5)
MAX_SESSION_START = int(1e6)

MIN_SESSION_END = 0
MAX_SESSION_END = int(1e3)

fake = Faker()
SEPARATOR = "-" * 80


def log_error(msg: str) -> None:
    print(f"[red]ERROR[/red]: {msg}")


def log_info(msg: str) -> None:
    print(f"[cyan]INFO[/cyan]: {msg}")


@dataclass
class Channel:
    id: str
    name: str
    url: str
    is_subscribed: bool
    subscribers_count: int
    added_at: int

    def __init__(self) -> None:
        self.id = fake.uuid4()
        self.name = fake.name()
        self.url = f"https://www.youtube.com/@{self.id}"
        self.is_subscribed = fake.pybool()
        self.subscribers_count = fake.pyint(min_value=10, max_value=1000000)
        self.added_at = int(time.time())

    def exists(self, cursor: sqlite3.Cursor):
        cursor.execute("SELECT id FROM channels WHERE id = ?", (self.id,))
        return len(cursor.fetchall()) > 0

    def insert(self, cursor: sqlite3.Cursor):
        cursor.execute(
            "INSERT INTO channels (id, name, url, is_subscribed, subscribers_count, added_at) VALUES (?, ?, ?, ?, ?, ?)",
            (
                self.id,
                self.name,
                self.url,
                self.is_subscribed,
                self.subscribers_count,
                self.added_at,
            ),
        )


@dataclass
class Video:
    id: str
    channel_id: str
    url: str
    title: str
    description: str
    watch_counter: int
    duration_seconds: int
    likes_count: int
    view_count: int
    comments_count: int
    published_at: int
    added_at: int

    def __init__(self, channel_id: str) -> None:
        self.id = fake.uuid4()
        self.channel_id = channel_id
        self.url = f"https://www.youtube.com/watch?v={self.id}"
        self.title = fake.sentence()
        self.description = fake.text()
        self.watch_counter = 0
        self.duration_seconds = fake.pyint(min_value=1, max_value=3600)
        self.likes_count = fake.pyint(min_value=0, max_value=10000)
        self.view_count = fake.pyint(min_value=0, max_value=100000)
        self.comments_count = fake.pyint(min_value=0, max_value=10000)
        self.published_at = int(time.time()) - fake.pyint(
            min_value=MIN_VIDEO_PUBLISHED_AT, max_value=MAX_VIDEO_PUBLISHED_AT
        )
        self.added_at = int(time.time())

    def exists(self, cursor: sqlite3.Cursor):
        cursor.execute("SELECT id FROM videos WHERE id = ?", (self.id,))
        return len(cursor.fetchall()) > 0

    def insert(self, cursor: sqlite3.Cursor):
        cursor.execute(
            """INSERT INTO videos
            (
                id,
                channel_id,
                url,
                title,
                description,
                watch_counter,
                duration_seconds,
                likes_count,
                view_count,
                comments_count,
                published_at,
                added_at
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)""",
            (
                self.id,
                self.channel_id,
                self.url,
                self.title,
                self.description,
                self.watch_counter,
                self.duration_seconds,
                self.likes_count,
                self.view_count,
                self.comments_count,
                self.published_at,
                self.added_at,
            ),
        )


@dataclass
class Tag:
    id: str
    name: str
    added_at: int

    def __init__(self) -> None:
        self.id = fake.uuid4()
        self.name = fake.word()
        self.added_at = int(time.time())

    def exists(self, cursor: sqlite3.Cursor):
        cursor.execute("SELECT name FROM tags WHERE name = ?", (self.name,))
        return len(cursor.fetchall()) > 0

    def insert(self, cursor: sqlite3.Cursor):
        cursor.execute(
            "INSERT INTO tags (id, name, added_at) VALUES (?, ?, ?)",
            (self.id, self.name, self.added_at),
        )


@dataclass
class VideoTag:
    video_id: str
    tag_id: str

    def __init__(self, video_id: str, tag_id: str) -> None:
        self.video_id = video_id
        self.tag_id = tag_id

    def exists(self, cursor: sqlite3.Cursor):
        cursor.execute(
            "SELECT video_id, tag_id FROM video_tags WHERE video_id = ? AND tag_id = ?",
            (self.video_id, self.tag_id),
        )
        return len(cursor.fetchall()) > 0

    def insert(self, cursor: sqlite3.Cursor):
        cursor.execute(
            "INSERT INTO video_tags (video_id, tag_id) VALUES (?, ?)",
            (self.video_id, self.tag_id),
        )


@dataclass
class WatchHistory:
    id: str
    video_id: str
    channel_id: str
    watch_duration_seconds: int
    session_start_date: int
    session_end_date: int
    added_at: int

    def __init__(self, video: Video, channel_id: str) -> None:
        timestamp = int(time.time())
        start = video.published_at + fake.pyint(
            min_value=MIN_SESSION_START, max_value=MAX_SESSION_START
        )
        end = start + fake.pyint(min_value=MIN_SESSION_END, max_value=MAX_SESSION_END)
        duration = (end - start) + fake.pyint(min_value=0, max_value=500)

        self.id = fake.uuid4()
        self.video_id = video.id
        self.channel_id = channel_id
        self.watch_duration_seconds = duration
        self.session_start_date = start
        self.session_end_date = end
        self.added_at = timestamp

    def exists(self, cursor: sqlite3.Cursor):
        cursor.execute("SELECT id FROM watch_history WHERE id = ?", (self.id,))
        return len(cursor.fetchall()) > 0

    def insert(self, cursor: sqlite3.Cursor):
        cursor.execute(
            """INSERT INTO watch_history
            (
                    id,
                    video_id,
                    channel_id,
                    watch_duration_seconds,
                    session_start_date,
                    session_end_date,
                    added_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?)""",
            (
                self.id,
                self.video_id,
                self.channel_id,
                self.watch_duration_seconds,
                self.session_start_date,
                self.session_end_date,
                self.added_at,
            ),
        )


def main() -> int:
    default_db_file = "../dev-data/chianti.db"

    parser = argparse.ArgumentParser(
        description="Generate random data for testing purposes"
    )
    parser.add_argument(
        "--db-file", help="Path to the SQLite database file", default=default_db_file
    )
    parser.add_argument(
        "--channels",
        help="Number of channels to generate",
        type=int,
        default=10,
    )
    parser.add_argument(
        "--videos",
        help="Max number of videos to generate per channel",
        type=int,
        default=10,
    )
    parser.add_argument(
        "--tags",
        help="Max number of tags to generate per video",
        type=int,
        default=5,
    )
    parser.add_argument(
        "--watch-history",
        help="Max number of watch history entries to generate per video",
        type=int,
        default=10,
    )

    args = parser.parse_args()

    db_file = args.db_file
    total_channels = args.channels
    max_videos = args.videos
    max_tags = args.tags
    max_watch_history = args.watch_history

    if not os.path.exists(db_file):
        log_error(f"Database file '{db_file}'' doesn't exist")
        return 1

    log_info(f"Using database file '{db_file}'")
    conn = sqlite3.connect(db_file)

    cursor = conn.cursor()

    print(SEPARATOR)
    log_info(f"Generating {total_channels} channels")
    print(SEPARATOR)
    for _ in range(total_channels):
        channel = Channel()
        total_videos = randrange(1, max_videos)
        print(f"Channel(videos: {total_videos}) '{channel.name}'")

        if not channel.exists(cursor):
            channel.insert(cursor)

        for _ in range(total_videos):
            video = Video(channel.id)
            total_tags = randrange(0, max_tags)
            total_watch_history = randrange(1, max_watch_history)
            print(
                f"\tVideo(watch_history: {total_watch_history}, tags: {total_tags}) '{video.title}'"
            )

            if not video.exists(cursor):
                video.insert(cursor)

            for _ in range(total_watch_history):
                watch_history = WatchHistory(video, channel.id)

                if not watch_history.exists(cursor):
                    watch_history.insert(cursor)

            for _ in range(total_tags):
                tag = Tag()
                video_tag = VideoTag(video.id, tag.id)
                print(f"\t\tTag '{tag.name}'")

                if not tag.exists(cursor):
                    tag.insert(cursor)

                if not video_tag.exists(cursor):
                    video_tag.insert(cursor)
    print(SEPARATOR)

    conn.commit()
    conn.close()

    return 0


if __name__ == "__main__":
    sys.exit(main())
