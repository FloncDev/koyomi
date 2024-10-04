create table if not exists timetabled_lessons (
    id serial primary key,
    subject varchar(255) not null,
    teachers text not null,
    location varchar(32) not null,
    start time not null,
    "end" time not null,
    -- 0 = Monday, 6 = Sunday
    weekday smallint not null
);
