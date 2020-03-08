Create TYPE type as ENUM ('normal', 'warmup', 'cooldown', 'drop', 'fail')

CREATE TABLE set {
    id integer PRIMARY KEY,
    exercise_id int null,
    type type not null,
    unit varchar null,
    goal_reps smallint,
    goal_value varchar,
    description varchar,
    created_or_completed timestamp,
    completed_reps smallint null,
    completed_value varchar null
}
