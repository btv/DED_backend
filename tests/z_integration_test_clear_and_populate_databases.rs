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

    static EXERCISES: &str = "1,abdominals,ab crunch machine,auto populated exercise\
     ;2,abdominals,ab roller,auto populated exercise\
     ;3,adductors,adductor,auto populated exercise\
     ;4,adductors,adductor/groin,auto populated exercise\
     ;5,abdominals,advanced kettlebell windmill,auto populated exercise\
     ;6,abdominals,air bike,auto populated exercise\
     ;7,quadriceps,all fours quad stretch,auto populated exercise\
     ;8,biceps,alternate hammer curl,auto populated exercise\
     ;9,abdominals,alternate heel touchers,auto populated exercise\
     ;10,biceps,alternate incline dumbbell curl,auto populated exercise\
     ;11,quadriceps,alternate leg diagonal bound,auto populated exercise\
     ;12,shoulders,alternating cable shoulder press,auto populated exercise\
     ;13,shoulders,alternating deltoid raise,auto populated exercise\
     ;14,chest,alternating floor press,auto populated exercise\
     ;15,hamstrings,alternating hang clean,auto populated exercise\
     ;16,shoulders,alternating kettlebell press,auto populated exercise\
     ;17,middle back,alternating kettlebell row,auto populated exercise\
     ;18,middle back,alternating renegade row,auto populated exercise\
     ;19,calves,ankle circles,auto populated exercise\
     ;20,glutes,ankle on the knee,auto populated exercise\
     ;21,calves,anterior tibialis-smr,auto populated exercise\
     ;22,shoulders,anti-gravity press,auto populated exercise\
     ;23,shoulders,arm circles,auto populated exercise\
     ;24,shoulders,arnold dumbbell press,auto populated exercise\
     ;25,chest,around the worlds,auto populated exercise\
     ;26,lower back,atlas stone trainer,auto populated exercise\
     ;27,lower back,atlas stones,auto populated exercise\
     ;28,lower back,axle deadlift,auto populated exercise\
     ;29,abdominals,ab crunch machine,auto populated exercise\
     ;30,abdominals,ab roller,auto populated exercise";

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
            match newExercise.create(&conn) {
                Err(E) => {
                    assert_eq!(E, diesel::NotFound);
                }
                Ok(_T) => {
                    assert_eq!(_T.description, items[1].to_string());
                }
            }
        }
    }
}
