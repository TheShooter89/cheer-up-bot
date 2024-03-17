CREATE TABLE IF NOT EXISTS users (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    telegram_id INTEGER NOT NULL,
    username TEXT NOT NULL,
    first_name TEXT NOT NULL,
    last_name TEXT,
    UNIQUE(username)
);

CREATE TABLE IF NOT EXISTS notes (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    file_name TEXT NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id)
);
