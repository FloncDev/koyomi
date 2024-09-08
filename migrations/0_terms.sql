create table if not exists terms (
    id serial primary key,
    name varchar(16) not null,
    start date not null,
    "end" date not null
);
