CREATE TABLE IF NOT EXISTS sets (
    id SERIAL PRIMARY KEY,
    exercise_id int not null,
    style varchar not null,
    unit varchar not null,
    goal_reps smallint not null ,
    goal_value smallint not null ,
    description varchar not null,
    created_or_completed timestamp not null DEFAULT NOW(),
    completed_reps smallint not null DEFAULT 0,
    completed_value smallint not null DEFAULT 0,
    origin_id int not null
);



CREATE TABLE IF NOT EXISTS exercises(
    id SERIAL PRIMARY KEY ,
    origin_id INTEGER not null,
    workout_id INTEGER  not NULL,
    name varchar not null,
    exercise_type varchar not null,
    description varchar not null,
    notes varchar  not null,
    create_time timestamp not null DEFAULT NOW(),
    complete_time timestamp not null DEFAULT NOW()
);


CREATE  TABLE  IF NOT EXISTS users(
    id SERIAL PRIMARY KEY,
    username varchar not null,
    name varchar not null,
    email varchar not null,
    passwd varchar not null,
    unique(username),
    unique(email)
);

CREATE TABLE IF NOT EXISTS workouts (
    id SERIAL PRIMARY KEY,
    origin_id integer not null,
    name varchar not null,
    user_id integer not null,
    description varchar not null,
    notes varchar not null ,
    created_time timestamp not null DEFAULT NOW(),
    completed_time timestamp not null DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS rawdata (
    id SERIAL PRIMARY KEY,
    create_time timestamp  not null DEFAULT NOW(),
    blob bytea,
    file_size bigint
);


CREATE TABLE IF NOT EXISTS processlog (
    id SERIAL PRIMARY KEY,
    create_time timestamp  not null DEFAULT NOW(),
    process_time timestamp  not null DEFAULT NOW(),
    log varchar not null
);
