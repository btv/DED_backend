CREATE TABLE sets (
    id Integer PRIMARY KEY,
    exercise_id int null,
    style varchar not null,
    unit varchar null,
    goal_reps smallint,
    goal_value varchar,
    description varchar null,
    created_or_completed timestamp,
    completed_reps smallint null,
    completed_value varchar null
)

CREATE TABLE exercises(
    id INTEGER PRIMARY KEY ,
    origin_id INTEGER  null,
    set_id INTEGER  not NULL,
    fname varchar not null,
    exercise_type INTEGER not null,
    description varchar not null,
    notes varchar null,
    create_time timestamp ,
    complete_time timestamp ,
    create_id INTEGER not null,
    completed_id INTEGER not null
)


CREATE  TABLE  user(
    id INTEGER PRIMARY KEY,
    username varchar not null,
    fname varchar not null,
    email varchar not null,
    salt varchar not null,
)

CREATE TABLE  workout (
    id INTEGER PRIMARY KEY,
    origin_id integer null,
    exercise INTEGER not null,
    uname varchar not null,
    description varchar not null,
    notes varchar,
    created_time timestamp ,
    completed_time timestamp ,
    created_id integer not null,
    completed_id integer not null,
)