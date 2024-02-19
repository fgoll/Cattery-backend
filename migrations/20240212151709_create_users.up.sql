create table users (
    id integer primary key not null,
    openid text not null,
    session_key text not null,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

create unique index users_openid_index on users(openid);

create table admin_users (
    id integer primary key not null,
    phone text not null,
    password text not null,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

create unique index admin_users_phone_index on admin_users(password);