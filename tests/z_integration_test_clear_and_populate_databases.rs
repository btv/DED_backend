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
     ;30,abdominals,ab roller,auto populated exercise\
     ;31,adductors,adductor,auto populated exercise\
     ;32,adductors,adductor/groin,auto populated exercise\
     ;33,abdominals,advanced kettlebell windmill,auto populated exercise\
     ;34,abdominals,air bike,auto populated exercise\
     ;35,quadriceps,all fours quad stretch,auto populated exercise\
     ;36,biceps,alternate hammer curl,auto populated exercise\
     ;37,abdominals,alternate heel touchers,auto populated exercise\
     ;38,biceps,alternate incline dumbbell curl,auto populated exercise\
     ;39,quadriceps,alternate leg diagonal bound,auto populated exercise\
     ;40,shoulders,alternating cable shoulder press,auto populated exercise\
     ;41,shoulders,alternating deltoid raise,auto populated exercise\
     ;42,chest,alternating floor press,auto populated exercise\
     ;43,hamstrings,alternating hang clean,auto populated exercise\
     ;44,shoulders,alternating kettlebell press,auto populated exercise\
     ;45,middle back,alternating kettlebell row,auto populated exercise\
     ;46,middle back,alternating renegade row,auto populated exercise\
     ;47,calves,ankle circles,auto populated exercise\
     ;48,glutes,ankle on the knee,auto populated exercise\
     ;49,calves,anterior tibialis-smr,auto populated exercise\
     ;50,shoulders,anti-gravity press,auto populated exercise\
     ;51,shoulders,arm circles,auto populated exercise\
     ;52,shoulders,arnold dumbbell press,auto populated exercise\
     ;53,chest,around the worlds,auto populated exercise\
     ;54,lower back,atlas stone trainer,auto populated exercise\
     ;55,lower back,atlas stones,auto populated exercise\
     ;56,lower back,axle deadlift,auto populated exercise\
     ;57,shoulders,back flyes - with bands,auto populated exercise\
     ;58,quadriceps,backward drag,auto populated exercise\
     ;59,shoulders,backward medicine ball throw,auto populated exercise\
     ;60,calves,balance board,auto populated exercise\
     ;61,hamstrings,ball leg curl,auto populated exercise\
     ;62,lats,band assisted pull-up,auto populated exercise\
     ;63,hamstrings,band good morning,auto populated exercise\
     ;64,hamstrings,band good morning (pull through),auto populated exercise\
     ;65,adductors,band hip adductions,auto populated exercise\
     ;66,shoulders,band pull apart,auto populated exercise\
     ;67,triceps,band skull crusher,auto populated exercise\
     ;68,abdominals,barbell ab rollout,auto populated exercise\
     ;69,abdominals,barbell ab rollout - on knees,auto populated exercise\
     ;70,chest,barbell bench press - medium grip,auto populated exercise\
     ;71,biceps,barbell curl,auto populated exercise\
     ;72,biceps,barbell curls lying against an incline,auto populated exercise\
     ;73,lower back,barbell deadlift,auto populated exercise\
     ;74,quadriceps,barbell full squat,auto populated exercise\
     ;75,glutes,barbell glute bridge,auto populated exercise\
     ;76,chest,barbell guillotine bench press,auto populated exercise\
     ;77,quadriceps,barbell hack squat,auto populated exercise\
     ;78,glutes,barbell hip thrust,auto populated exercise\
     ;79,chest,barbell incline bench press - medium grip,auto populated exercise\
     ;80,shoulders,barbell incline shoulder raise,auto populated exercise\
     ;81,quadriceps,barbell lunge,auto populated exercise\
     ;82,shoulders,barbell rear delt row,auto populated exercise\
     ;83,abdominals,barbell rollout from bench,auto populated exercise\
     ;84,calves,barbell seated calf raise,auto populated exercise\
     ;85,shoulders,barbell shoulder press,auto populated exercise\
     ;86,traps,barbell shrug,auto populated exercise\
     ;87,traps,barbell shrug behind the back,auto populated exercise\
     ;88,abdominals,barbell side bend,auto populated exercise\
     ;89,quadriceps,barbell side split squat,auto populated exercise\
     ;90,quadriceps,barbell squat,auto populated exercise\
     ;91,quadriceps,barbell squat to a bench,auto populated exercise\
     ;92,quadriceps,barbell step ups,auto populated exercise\
     ;93,quadriceps,barbell walking lunge,auto populated exercise\
     ;94,shoulders,battling ropes,auto populated exercise\
     ;95,quadriceps,bear crawl sled drags,auto populated exercise\
     ;96,chest,behind head chest stretch,auto populated exercise\
     ;97,triceps,bench dips,auto populated exercise\
     ;98,quadriceps,bench jump,auto populated exercise\
     ;99,triceps,bench press - powerlifting,auto populated exercise\
     ;100,chest,bench press - with bands,auto populated exercise\
     ;101,triceps,bench press with chains,auto populated exercise\
     ;102,quadriceps,bench sprint,auto populated exercise\
     ;103,middle back,bent over barbell row,auto populated exercise\
     ;104,shoulders,bent over dumbbell rear delt raise with head on bench,auto populated exercise\
     ;105,shoulders,bent over low-pulley side lateral,auto populated exercise\
     ;106,middle back,bent over one-arm long bar row,auto populated exercise\
     ;107,middle back,bent over two-arm long bar row,auto populated exercise\
     ;108,middle back,bent over two-dumbbell row,auto populated exercise\
     ;109,middle back,bent over two-dumbbell row with palms in,auto populated exercise\
     ;110,abdominals,bent press,auto populated exercise\
     ;111,lats,bent-arm barbell pullover,auto populated exercise\
     ;112,chest,bent-arm dumbbell pullover,auto populated exercise\
     ;113,abdominals,bent-knee hip raise,auto populated exercise\
     ;114,quadriceps,bicycling,auto populated exercise\
     ;115,quadriceps,bicycling stationary,auto populated exercise\
     ;116,triceps,board press,auto populated exercise\
     ;117,triceps,body tricep press,auto populated exercise\
     ;118,triceps,body-up,auto populated exercise\
     ;119,chest,bodyweight flyes,auto populated exercise\
     ;120,middle back,bodyweight mid row,auto populated exercise\
     ;121,quadriceps,bodyweight squat,auto populated exercise\
     ;122,quadriceps,bodyweight walking lunge,auto populated exercise\
     ;123,abdominals,bosu ball cable crunch with side bends,auto populated exercise\
     ;124,abdominals,bottoms up,auto populated exercise\
     ;125,forearms,bottoms-up clean from the hang position,auto populated exercise\
     ;126,hamstrings,box jump (multiple response),auto populated exercise\
     ;127,hamstrings,box skip,auto populated exercise\
     ;128,quadriceps,box squat,auto populated exercise\
     ;129,quadriceps,box squat with bands,auto populated exercise\
     ;130,quadriceps,box squat with chains,auto populated exercise\
     ;131,biceps,brachialis-smr,auto populated exercise\
     ;132,shoulders,bradford/rocky presses,auto populated exercise\
     ;133,glutes,butt lift (bridge),auto populated exercise\
     ;134,abdominals,butt-ups,auto populated exercise\
     ;135,chest,butterfly,auto populated exercise\
     ;136,chest,cable chest press,auto populated exercise\
     ;137,chest,cable crossover,auto populated exercise\
     ;138,abdominals,cable crunch,auto populated exercise\
     ;139,quadriceps,cable deadlifts,auto populated exercise\
     ;140,biceps,cable hammer curls - rope attachment,auto populated exercise\
     ;141,quadriceps,cable hip adduction,auto populated exercise\
     ;142,lats,cable incline pushdown,auto populated exercise\
     ;143,triceps,cable incline triceps extension,auto populated exercise\
     ;144,shoulders,cable internal rotation,auto populated exercise\
     ;145,chest,cable iron cross,auto populated exercise\
     ;146,abdominals,cable judo flip,auto populated exercise\
     ;147,triceps,cable lying triceps extension,auto populated exercise\
     ;148,triceps,cable one arm tricep extension,auto populated exercise\
     ;149,biceps,cable preacher curl,auto populated exercise\
     ;150,shoulders,cable rear delt fly,auto populated exercise\
     ;151,abdominals,cable reverse crunch,auto populated exercise\
     ;152,triceps,cable rope overhead triceps extension,auto populated exercise\
     ;153,shoulders,cable rope rear-delt rows,auto populated exercise\
     ;154,abdominals,cable russian twists,auto populated exercise\
     ;155,abdominals,cable seated crunch,auto populated exercise\
     ;156,shoulders,cable seated lateral raise,auto populated exercise\
     ;157,shoulders,cable shoulder press,auto populated exercise\
     ;158,traps,cable shrugs,auto populated exercise\
     ;159,abdominals,cable tuck reverse crunch,auto populated exercise\
     ;160,forearms,cable wrist curl,auto populated exercise\
     ;161,calves,calf press,auto populated exercise\
     ;162,calves,calf press on the leg press machine,auto populated exercise\
     ;163,calves,calf raise on a dumbbell,auto populated exercise\
     ;164,calves,calf raises - with bands,auto populated exercise";

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
                workout_id: 0,
                name: "auto generated".to_string(),
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
