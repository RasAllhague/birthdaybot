DROP TABLE IF EXISTS birthday;
DROP TABLE IF EXISTS subscription;

CREATE TABLE birthday (
    id_birthday SERIAL,
    guild_id BIGINT NOT NULL,
    user_id BIGINT NOT NULL,
    date TIMESTAMP NOT NULL,
    create_date TIMESTAMP NOT NULL,
    modify_date TIMESTAMP,
    PRIMARY KEY (id_birthday),
    UNIQUE(user_id),
    UNIQUE(guild_id)
);

CREATE TABLE subscription (
    id_subscription SERIAL,
    guild_id BIGINT NOT NULL,
    user_id BIGINT NOT NULL,
    birthday_id INTEGER NOT NULL,
    create_date TIMESTAMP NOT NULL,
    modify_date TIMESTAMP,
    PRIMARY KEY (id_subscription),
    UNIQUE(user_id),
    UNIQUE(guild_id),
    FOREIGN KEY (birthday_id) REFERENCES birthday(id_birthday)
);