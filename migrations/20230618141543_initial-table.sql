-- Add migration script here
CREATE TABLE IF NOT EXISTS cities (
    city TEXT NOT NULL,
    city_ascii TEXT NOT NULL,
    coords GEOMETRY(Point, 4326),
    country TEXT NOT NULL,
    iso2 TEXT,
    iso3 TEXT,
    admin_name TEXT,
    capital TEXT,
    population INT NOT NULL,
    id BIGINT PRIMARY KEY
);
