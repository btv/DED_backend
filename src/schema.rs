table! {
    sets(id) {
        id -> Integer,
        exercise_id -> Nullable<Integer>,
        style -> VarChar,
        unit -> Nullable<VarChar>,
        goal_reps -> SmallInt,
        goal_value -> VarChar,
        description -> Nullable<VarChar>,
        created_or_completed -> Nullable<Timestamp>,
        completed_reps -> Nullable<SmallInt>,
        completed_value -> Nullable<VarChar>,
    }
    exercise(id){
        id ->Integer,
        origin_id ->int,
        set_id -> Integer,
        uname -> VarChar,
        exercise_type -> Integer,
        description -> VarChar,
        notes ->VarChar,
        create_time -> Nullable <Timestamp>,
        complete_time -> Nullable <Timestamp>,
        created_id ->Integer,
        completed_id ->Integer,
    }

    user(id){
        id ->Integer,
        username -> Nullable<VarChar>,
        fname -> Nullable<VarChar>,
        email -> Nullable<VarChar>,
        salt -> Nullable<Varchar>,
    }

    workout(id){}
        id ->Integer,
        origin_id ->Integer,
        exercise -> Integer,
        uname ->Nullable<VarChar>,
        description ->Nullable<VarChar>,
        notes -> Nullable<VarChar>,
        created_time -> Nullable<Timestamp>,
        completed_time -> Nullable<Timestamp>,
        created_id ->Integer,
        completed_id ->Integer,
}
