CREATE TABLE IF NOT EXISTS sets (
    id Integer PRIMARY KEY,
    exercise_id int,
    style varchar not null,
    unit varchar not null,
    goal_reps smallint not null ,
    goal_value varchar not null ,
    description varchar not null,
    created_or_completed timestamp not null ,
    completed_reps smallint not null,
    completed_value varchar not null
);



CREATE TABLE IF NOT EXISTS exercises(
    id INTEGER PRIMARY KEY ,
    origin_id INTEGER not null,
    set_id INTEGER  not NULL,
    fname varchar not null,
    exercise_type INTEGER not null,
    description varchar not null,
    notes varchar  not null,
    create_time timestamp not null,
    complete_time timestamp not null,
    create_id INTEGER not null,
    completed_id INTEGER not null
);


CREATE  TABLE  IF NOT EXISTS users(
    id INTEGER PRIMARY KEY,
    username varchar not null,
    fname varchar not null,
    email varchar not null,
    salt varchar not null
);

CREATE TABLE IF NOT EXISTS workouts (
    id INTEGER PRIMARY KEY,
    origin_id integer not null,
    exercise INTEGER not null,
    fname varchar not null,
    description varchar not null,
    notes varchar not null ,
    created_time timestamp not null,
    completed_time timestamp not null  ,
    create_id integer not null,
    completed_id integer not null
);