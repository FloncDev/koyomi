create table if not exists lessons (
    id serial primary key,
    timetabled_lesson_id integer references timetabled_lessons not null ,
    uid uuid not null
);
