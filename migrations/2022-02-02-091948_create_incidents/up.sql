CREATE SCHEMA IF NOT EXISTS acled;

-- Your SQL goes here
CREATE TABLE acled.incidents (
    data_id BIGINT PRIMARY KEY,
    iso BIGINT NOT NULL,
    event_id_cnty VARCHAR NOT NULL,
    event_id_no_cnty BIGINT NOT NULL,
    event_date DATE NOT NULL,
    year BIGINT NOT NULL,
    time_precision BIGINT NOT NULL,
    event_type VARCHAR NOT NULL,
    sub_event_type VARCHAR NOT NULL,
    actor1 VARCHAR NOT NULL,
    assoc_actor_1 VARCHAR NOT NULL,
    inter1 BIGINT NOT NULL,
    actor2 VARCHAR NOT NULL,
    assoc_actor_2 VARCHAR NOT NULL,
    inter2 BIGINT NOT NULL,
    interaction VARCHAR NOT NULL,
    region VARCHAR NOT NULL,
    country VARCHAR NOT NULL,
    admin1 VARCHAR NOT NULL,
    admin2 VARCHAR NOT NULL,
    admin3 VARCHAR NOT NULL,
    location VARCHAR NOT NULL,
    geo_precision BIGINT NOT NULL,
    source VARCHAR NOT NULL,
    source_scale VARCHAR NOT NULL,
    notes VARCHAR NOT NULL,
    fatalities BIGINT NOT NULL,
    timestamp BIGINT NOT NULL,
    iso3 VARCHAR NOT NULL,
    geom GEOMETRY(POINT, 4326) NOT NULL
);

CREATE INDEX iso3_idx ON acled.incidents (iso3);