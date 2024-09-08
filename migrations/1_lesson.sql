create table if not exists lessons (
    id serial primary key,
    subject_id integer references subjects,
    teachers text not null,
    location varchar(8) not null,
    start timestamp not null,
    "end" timestamp not null,
    uid uuid not null
);
