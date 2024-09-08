create table if not exists timetabled_lessons (
    id serial primary key,
    subject_id integer references subjects not null,
    teachers text not null,
    location varchar(8) not null,
    start time not null,
    "end" time not null,
    -- 0 = Monday, 6 = Sunday
    weekday smallint not null
);
