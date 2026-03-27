-- Add migration script here

create table users (
    id serial primary key not null,
    username varchar(64) not null,
    uuid varchar(256) not null,
    pw_hash varchar(1024) not null,
    created_at timestamp with time zone default current_timestamp,
    updated_at timestamp with time zone default current_timestamp
);

create unique index users_username on users(username);
create unique index users_uuid on users(uuid);
