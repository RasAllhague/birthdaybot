CREATE TABLE IF NOT EXISTS send_notifications(
    id_send_notification SERIAL,
    subscription_id INTEGER NOT NULL,
    current_year INTEGER NOT NULL,
    create_date TIMESTAMP NOT NULL,
    PRIMARY KEY (id_send_notification),
    UNIQUE (subscription_id, current_year)
);