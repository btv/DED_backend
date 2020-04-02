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
    ;4,Free Weight Bench Press,Weight belt optional;5,Smith machine BP, add 10% to weight\
    ;6,Incline BP, Olympic Bar & weight belt;7,Calf lift,Multiple reps\
    ;7,Leg Extensions,add 10%;8,Pull up,Military style;9,Pull up,Spread Arms";

    #[test]
    fn test_base_exercise_integration() {
        let conn = establish_connection().get().unwrap();
        let _xxx = diesel::delete(DED_backend::schema::exercises::dsl::exercises)
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