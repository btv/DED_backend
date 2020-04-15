CREATE TABLE IF NOT EXISTS sets (
    id SERIAL PRIMARY KEY,
    exercise_id int not null,
    style varchar not null,
    unit varchar not null,
    goal_reps smallint not null ,
    goal_value varchar not null ,
    description varchar not null,
    created_or_completed timestamp not null DEFAULT NOW(),
    completed_reps smallint not null DEFAULT 0,
    completed_value varchar not null DEFAULT ''
);



CREATE TABLE IF NOT EXISTS exercises(
    id SERIAL PRIMARY KEY ,
    origin_id INTEGER not null,
    set_id INTEGER  not NULL,
    fname varchar not null,
    exercise_type INTEGER not null,
    description varchar not null,
    notes varchar  not null,
    create_time timestamp not null DEFAULT NOW(),
    complete_time timestamp not null DEFAULT NOW(),
    create_id INTEGER not null DEFAULT 0,
    completed_id INTEGER not null DEFAULT 0
);


CREATE  TABLE  IF NOT EXISTS users(
    id SERIAL PRIMARY KEY,
    username varchar not null,
    fname varchar not null,
    email varchar not null,
    passwd varchar not null,
    unique(username),
    unique(email)
);

CREATE TABLE IF NOT EXISTS workouts (
    id SERIAL PRIMARY KEY,
    origin_id integer not null,
    exercise INTEGER not null,
    fname varchar not null,
    description varchar not null,
    notes varchar not null ,
    created_time timestamp not null DEFAULT NOW(),
    completed_time timestamp not null DEFAULT NOW(),
    create_id integer not null DEFAULT 0,
    completed_id integer not null DEFAULT 0
);
