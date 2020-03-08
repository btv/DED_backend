Create TYPE style as ENUM ('normal', 'warmup', 'cooldown', 'drop', 'fail')

CREATE TABLE sets {
    id integer PRIMARY KEY,
    exercise_id int null,
    style style,
    unit varchar null,
    goal_reps smallint,
    goal_value varchar,
    description varchar,
    created_or_completed timestamp,
    completed_reps smallint null,
    completed_value varchar null
}
