create table if not exists lessons (
    id serial primary key,
    subject varchar(255) not null,
    teachers text not null,
    location varchar(32) not null,
    start timestamp with time zone not null,
    "end" timestamp with time zone not null
)
