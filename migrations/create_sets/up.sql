CREATE TABLE sets {
    id integer PRIMARY KEY,
    exercise_id int null,
    type varchar null,
    unit varchar null,
    goal_reps smallint,
    goal_value varchar,
    description varchar,
    created_or_completed
    completed_reps smallint null,
    completed_value varchar null
}
