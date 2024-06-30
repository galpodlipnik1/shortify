create table UrlMapping (
    id serial primary key,
    url text not null,
    key text not null,
    created_at timestamp not null default current_timestamp
);

CREATE UNIQUE INDEX idx_short_url ON UrlMapping (key);