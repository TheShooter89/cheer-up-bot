CREATE TABLE IF NOT EXISTS users (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    telegram_id INTEGER,
    username TEXT,
    first_name TEXT,
    last_name TEXT,
    UNIQUE(username)
);

CREATE TABLE IF NOT EXISTS notes (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    file_name TEXT NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id)
);
