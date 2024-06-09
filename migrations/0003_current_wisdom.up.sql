CREATE TABLE IF NOT EXISTS current_wisdom (
    id SERIAL PRIMARY KEY,
    current_index INT NOT NULL DEFAULT 1,
    selected_wisdom VARCHAR(300)
);