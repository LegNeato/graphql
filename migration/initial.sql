DROP TABLE IF EXISTS point;
DROP TABLE IF EXISTS ride;
DROP TABLE IF EXISTS member;

CREATE TABLE member (
    id        UUID PRIMARY KEY,
    email     TEXT UNIQUE NOT NULL,
    firstname TEXT        NOT NULL,
    lastname  TEXT        NOT NULL,
    birthdate DATE        NOT NULL
);

CREATE TABLE ride (
    id          UUID PRIMARY KEY,
    rider       UUID      NOT NULL REFERENCES member (id),
    name        TEXT      NOT NULL,
    description TEXT      NULL,
    distance    INTEGER   NOT NULL,
    started     TIMESTAMP NOT NULL,
    ended       TIMESTAMP NOT NULL
);

CREATE TABLE point (
    ride      UUID          NOT NULL REFERENCES ride (id),
    id        SERIAL,
    latitude  NUMERIC(9, 6) NOT NULL,
    longitude NUMERIC(9, 6) NOT NULL,
    altitude  NUMERIC(7, 2) NOT NULL,
    timestamp TIMESTAMP     NOT NULL,

    PRIMARY KEY (ride, id)
);
