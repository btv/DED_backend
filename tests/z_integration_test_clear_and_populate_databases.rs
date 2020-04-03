#![allow(non_snake_case)]

extern crate diesel;
extern crate dotenv;

// NOTE
//   We are populating the DB for final use as an integration test
//   Although Rust test are run in parallell they are also run in alphabetical order.
//  Forcing this to be the last test with the z_prefix

mod tests {
    use DED_backend::establish_connection;
    use DED_backend::models::exercises::{Exercise, NewExercise};
    use diesel::RunQueryDsl;

    static EXERCISES: &str = "1,Free weight Squats,Olympic bar with weightbelt;2,Smith machine Squats,add 10% to weight\
    ;3,Incline machine Squats,Add 20% to weight;4,Single leg squats,use dumbbells\
    ;5,Free Weight Bench Press,Weight belt optional;6,Smith machine BP, add 10% to weight\
    ;7,Incline BP, Olympic Bar & weight belt;8,Calf lift,Multiple reps\
    ;9,Leg Extensions,add 10%;10,Pull up,Military style;11,Pull up,Spread Arms\
    ;12,Lunges,No weight;13,Lunges 10% bodyweight,Dumbells parallel to torso\
    ;14,Curls,dumbbells;15,Curls, curl bar standing;16,Curls, Sitting on curl bench with curl bar\
    ;17,Triceps, stading with curlbar;18,Rope Tricep Pushdown, use rope handle on cable station\
    ;19,Tricep dips, Use assist if not able to support body weight\
    ;20,Overhead Triceps Extension, sitting with dumbbells\
    ;21,Skullcrushers (Lying Triceps Extensions),use a curl bar\
    ;22,(ABS)Hardstyle plank, no weights;23,(ABS)Dead bug, meh;24,(ABS)Hollow extension-to-cannonball,more meh\
    ;25,(ABS)Dumbbell side bend, max 50% of bodyweight;26,(ABS)Barbell back squat, Olympic bar no weight\
    ;26,(FULLBODY) Inchworm,malarkey;27,(FULLBODY) TuckJump, looks silly;28,(FULLBODY) Bear Crawl, meh\
    ;29,(FULLBODY) Mountain climber,  Start on your hands and knees and die;30, (FULLBODY) Plyometric push-up,maybe this is a good one";

    #[test]
    fn test_base_exercise_integration() {
        let conn = establish_connection().get().unwrap();
        let _x = diesel::delete(DED_backend::schema::sets::dsl::sets)
            .execute(&conn);

        let _y = diesel::delete(DED_backend::schema::exercises::dsl::exercises)
            .execute(&conn);

        let _z = diesel::delete(DED_backend::schema::users::dsl::users)
            .execute(&conn);

        let _xxx = diesel::delete(DED_backend::schema::workouts::dsl::workouts)
            .execute(&conn);

        for token in EXERCISES.split(";") {
            let items: Vec<&str> = token.split(",").collect();
            let newExercise = NewExercise {
                origin_id: 1010,
                set_id: 0,
                fname: "auto generated".to_string(),
                exercise_type: items[0].to_string().parse::<i32>().unwrap(),
                description: items[1].to_string(),
                notes: items[2].to_string()
            };
            newExercise.create(&conn);
        }
    }



}