SET timezone = 'Europe/Moscow';
CREATE EXTENSION pgcrypto;

DROP TABLE IF EXISTS users CASCADE;
CREATE TABLE users
(
    id       SERIAL,
    username TEXT NOT NULL UNIQUE,
    password TEXT NOT NULL,
    PRIMARY KEY (id)
);

DROP TABLE IF EXISTS todos;
CREATE TABLE todos
(
    id            SERIAL,
    title         TEXT        NOT NULL,
    body          TEXT        NOT NULL,
    done          BOOLEAN     NOT NULL DEFAULT false,
    creation_time TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    user_id       INTEGER     NOT NULL,
    PRIMARY KEY (id),
    FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE
);

GRANT ALL PRIVILEGES ON TABLE users TO shurikdima;
GRANT ALL PRIVILEGES ON TABLE todos TO shurikdima;
GRANT ALL PRIVILEGES ON SEQUENCE users_id_seq TO shurikdima;
GRANT ALL PRIVILEGES ON SEQUENCE todos_id_seq TO shurikdima;
