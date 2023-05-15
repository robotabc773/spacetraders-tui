CREATE TABLE IF NOT EXISTS agents (
    account_id        text NOT NULL,
    symbol            text NOT NULL PRIMARY KEY,
    headquarters      text NOT NULL,
    credits           int NOT NULL
)
