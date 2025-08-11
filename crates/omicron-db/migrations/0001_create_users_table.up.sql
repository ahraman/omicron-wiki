CREATE TABLE users (
    user_id     BIGSERIAL PRIMARY KEY,
    user_name   VARCHAR(255) UNIQUE NOT NULL,
    user_email  VARCHAR(254) UNIQUE NOT NULL,

    user_password BYTEA NOT NULL,

    created_at  TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL
);
