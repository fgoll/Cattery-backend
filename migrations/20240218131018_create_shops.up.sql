-- Add up migration script here
create table shops (
    id integer primary key not null,
    app_id text not null,
    name text not null,
    content text not null,
    created_at TIMESTAMP not null default CURRENT_TIMESTAMP,
    updated_at TIMESTAMP not null default CURRENT_TIMESTAMP
);

create index shops_app_id_index on shops (app_id);