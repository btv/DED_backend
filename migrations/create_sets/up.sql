CREATE TABLE sets {
    id integer PRIMARY KEY,
    exercise_id int null,
    style varchar not null,
    unit varchar null,
    goal_reps smallint,
    goal_value varchar,
    description varchar null,
    created_or_completed timestamp,
    completed_reps smallint null,
    completed_value varchar null
}
