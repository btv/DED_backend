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
     ;164,calves,calf raises - with bands,auto populated exercise\
     ;165,calves,calf stretch elbows against wall,auto populated exercise\
     ;166,calves,calf stretch hands against wall,auto populated exercise\
     ;167,traps,calf-machine shoulder shrug,auto populated exercise\
     ;168,calves,calves-smr,auto populated exercise\
     ;169,quadriceps,car deadlift,auto populated exercise\
     ;170,shoulders,car drivers,auto populated exercise\
     ;171,adductors,carioca quick step,auto populated exercise\
     ;172,lower back,cat stretch,auto populated exercise\
     ;173,lats,catch and overhead throw,auto populated exercise\
     ;174,triceps,chain handle extension,auto populated exercise\
     ;175,chest,chain press,auto populated exercise\
     ;176,hamstrings,chair leg extended stretch,auto populated exercise\
     ;177,lats,chair lower back stretch,auto populated exercise\
     ;178,quadriceps,chair squat,auto populated exercise\
     ;179,shoulders,chair upper body stretch,auto populated exercise\
     ;180,chest,chest and front of shoulder stretch,auto populated exercise\
     ;181,chest,chest push (multiple response),auto populated exercise\
     ;182,chest,chest push (single response),auto populated exercise\
     ;183,chest,chest push from 3 point stance,auto populated exercise\
     ;184,chest,chest push with run release,auto populated exercise\
     ;185,chest,chest stretch on stability ball,auto populated exercise\
     ;186,lower back,child's pose,auto populated exercise\
     ;187,neck,chin to chest stretch,auto populated exercise\
     ;188,lats,chin-up,auto populated exercise\
     ;189,shoulders,circus bell,auto populated exercise\
     ;190,hamstrings,clean,auto populated exercise\
     ;191,shoulders,clean and jerk,auto populated exercise\
     ;192,shoulders,clean and press,auto populated exercise\
     ;193,hamstrings,clean deadlift,auto populated exercise\
     ;194,quadriceps,clean from blocks,auto populated exercise\
     ;195,quadriceps,clean pull,auto populated exercise\
     ;196,traps,clean shrug,auto populated exercise\
     ;197,chest,clock push-up,auto populated exercise\
     ;198,triceps,close-grip barbell bench press,auto populated exercise\
     ;199,triceps,close-grip dumbbell press,auto populated exercise\
     ;200,biceps,close-grip ez bar curl,auto populated exercise\
     ;201,biceps,close-grip ez-bar curl with band,auto populated exercise\
     ;202,triceps,close-grip ez-bar press,auto populated exercise\
     ;203,lats,close-grip front lat pulldown,auto populated exercise\
     ;204,triceps,close-grip push-up off of a dumbbell,auto populated exercise\
     ;205,biceps,close-grip standing barbell curl,auto populated exercise\
     ;206,abdominals,cocoons,auto populated exercise\
     ;207,quadriceps,conan's wheel,auto populated exercise\
     ;208,biceps,concentration curls,auto populated exercise\
     ;209,biceps,cross body hammer curl,auto populated exercise\
     ;210,chest,cross over - with bands,auto populated exercise\
     ;211,abdominals,cross-body crunch,auto populated exercise\
     ;212,lower back,crossover reverse lunge,auto populated exercise\
     ;213,shoulders,crucifix,auto populated exercise\
     ;214,abdominals,crunch - hands overhead,auto populated exercise\
     ;215,abdominals,crunch - legs on exercise ball,auto populated exercise\
     ;216,abdominals,crunches,auto populated exercise\
     ;217,shoulders,cuban press,auto populated exercise\
     ;218,lower back,dancer's stretch,auto populated exercise\
     ;219,lower back,deadlift with bands,auto populated exercise\
     ;220,lower back,deadlift with chains,auto populated exercise\
     ;221,chest,decline barbell bench press,auto populated exercise\
     ;222,triceps,decline close-grip bench to skull crusher,auto populated exercise\
     ;223,abdominals,decline crunch,auto populated exercise\
     ;224,chest,decline dumbbell bench press,auto populated exercise\
     ;225,chest,decline dumbbell flyes,auto populated exercise\
     ;226,triceps,decline dumbbell triceps extension,auto populated exercise\
     ;227,triceps,decline ez bar triceps extension,auto populated exercise\
     ;228,abdominals,decline oblique crunch,auto populated exercise\
     ;229,chest,decline push-up,auto populated exercise\
     ;230,abdominals,decline reverse crunch,auto populated exercise\
     ;231,chest,decline smith press,auto populated exercise\
     ;232,lower back,deficit deadlift,auto populated exercise\
     ;233,quadriceps,depth jump leap,auto populated exercise\
     ;234,triceps,dip machine,auto populated exercise\
     ;235,chest,dips - chest version,auto populated exercise\
     ;236,triceps,dips - triceps version,auto populated exercise\
     ;237,calves,donkey calf raises,auto populated exercise\
     ;238,hamstrings,double kettlebell alternating hang clean,auto populated exercise\
     ;239,shoulders,double kettlebell jerk,auto populated exercise\
     ;240,shoulders,double kettlebell push press,auto populated exercise\
     ;241,shoulders,double kettlebell snatch,auto populated exercise\
     ;242,abdominals,double kettlebell windmill,auto populated exercise\
     ;243,quadriceps,double leg butt kick,auto populated exercise\
     ;244,glutes,downward facing balance,auto populated exercise\
     ;245,biceps,drag curl,auto populated exercise\
     ;246,chest,drop push,auto populated exercise\
     ;247,biceps,dumbbell alternate bicep curl,auto populated exercise\
     ;248,chest,dumbbell bench press,auto populated exercise\
     ;249,chest,dumbbell bench press with neutral grip,auto populated exercise\
     ;250,biceps,dumbbell bicep curl,auto populated exercise\
     ;251,hamstrings,dumbbell clean,auto populated exercise\
     ;252,triceps,dumbbell floor press,auto populated exercise\
     ;253,chest,dumbbell flyes,auto populated exercise\
     ;254,middle back,dumbbell incline row,auto populated exercise\
     ;255,shoulders,dumbbell incline shoulder raise,auto populated exercise\
     ;256,quadriceps,dumbbell lunges,auto populated exercise\
     ;257,shoulders,dumbbell lying one-arm rear lateral raise,auto populated exercise\
     ;258,forearms,dumbbell lying pronation,auto populated exercise\
     ;259,shoulders,dumbbell lying rear lateral raise,auto populated exercise\
     ;260,forearms,dumbbell lying supination,auto populated exercise\
     ;261,shoulders,dumbbell one-arm shoulder press,auto populated exercise\
     ;262,triceps,dumbbell one-arm triceps extension,auto populated exercise\
     ;263,shoulders,dumbbell one-arm upright row,auto populated exercise\
     ;264,biceps,dumbbell prone incline curl,auto populated exercise\
     ;265,shoulders,dumbbell raise,auto populated exercise\
     ;266,quadriceps,dumbbell rear lunge,auto populated exercise\
     ;267,shoulders,dumbbell scaption,auto populated exercise\
     ;268,quadriceps,dumbbell seated box jump,auto populated exercise\
     ;269,calves,dumbbell seated one-leg calf raise,auto populated exercise\
     ;270,shoulders,dumbbell shoulder press,auto populated exercise\
     ;271,traps,dumbbell shrug,auto populated exercise\
     ;272,abdominals,dumbbell side bend,auto populated exercise\
     ;273,quadriceps,dumbbell squat,auto populated exercise\
     ;274,quadriceps,dumbbell squat to a bench,auto populated exercise\
     ;275,quadriceps,dumbbell step ups,auto populated exercise\
     ;276,triceps,dumbbell tricep extension -pronated grip,auto populated exercise\
     ;277,lats,dynamic back stretch,auto populated exercise\
     ;278,chest,dynamic chest stretch,auto populated exercise\
     ;279,shoulders,elbow circles,auto populated exercise\
     ;280,abdominals,elbow to knee,auto populated exercise\
     ;281,chest,elbows back,auto populated exercise\
     ;282,quadriceps,elevated back lunge,auto populated exercise\
     ;283,lats,elevated cable rows,auto populated exercise\
     ;284,quadriceps,elliptical trainer,auto populated exercise\
     ;285,abdominals,exercise ball crunch,auto populated exercise\
     ;286,abdominals,exercise ball pull-in,auto populated exercise\
     ;287,chest,extended range one-arm kettlebell floor press,auto populated exercise\
     ;288,shoulders,external rotation,auto populated exercise\
     ;289,shoulders,external rotation with band,auto populated exercise\
     ;290,shoulders,external rotation with cable,auto populated exercise\
     ;291,biceps,ez-bar curl,auto populated exercise\
     ;292,triceps,ez-bar skullcrusher,auto populated exercise\
     ;293,shoulders,face pull,auto populated exercise\
     ;294,forearms,farmer's walk,auto populated exercise\
     ;295,quadriceps,fast skipping,auto populated exercise\
     ;296,forearms,finger curls,auto populated exercise\
     ;297,chest,flat bench cable flyes,auto populated exercise\
     ;298,abdominals,flat bench leg pull-in,auto populated exercise\
     ;299,abdominals,flat bench lying leg raise,auto populated exercise\
     ;300,biceps,flexor incline dumbbell curls,auto populated exercise\
     ;301,hamstrings,floor glute-ham raise,auto populated exercise\
     ;302,triceps,floor press,auto populated exercise\
     ;303,triceps,floor press with chains,auto populated exercise\
     ;304,glutes,flutter kicks,auto populated exercise\
     ;305,calves,foot-smr,auto populated exercise\
     ;306,chest,forward drag with press,auto populated exercise\
     ;307,quadriceps,frankenstein squat,auto populated exercise\
     ;308,quadriceps,freehand jump squat,auto populated exercise\
     ;309,quadriceps,frog hops,auto populated exercise\
     ;310,abdominals,frog sit-ups,auto populated exercise\
     ;311,quadriceps,front barbell squat,auto populated exercise\
     ;312,quadriceps,front barbell squat to a bench,auto populated exercise\
     ;313,hamstrings,front box jump,auto populated exercise\
     ;314,shoulders,front cable raise,auto populated exercise\
     ;315,quadriceps,front cone hops (or hurdle hops),auto populated exercise\
     ;316,shoulders,front dumbbell raise,auto populated exercise\
     ;317,shoulders,front incline dumbbell raise,auto populated exercise\
     ;318,hamstrings,front leg raises,auto populated exercise\
     ;319,shoulders,front plate raise,auto populated exercise\
     ;320,chest,front raise and pullover,auto populated exercise\
     ;321,quadriceps,front squat (clean grip),auto populated exercise\
     ;322,quadriceps,front squats with two kettlebells,auto populated exercise\
     ;323,shoulders,front two-dumbbell raise,auto populated exercise\
     ;324,lats,full range-of-motion lat pulldown,auto populated exercise\
     ;325,lats,gironda sternum chins,auto populated exercise\
     ;326,hamstrings,glute ham raise,auto populated exercise\
     ;327,glutes,glute kickback,auto populated exercise\
     ;328,quadriceps,goblet squat,auto populated exercise\
     ;329,hamstrings,good morning,auto populated exercise\
     ;330,hamstrings,good morning off pins,auto populated exercise\
     ;331,abdominals,gorilla chin/crunch,auto populated exercise\
     ;332,adductors,groin and back stretch,auto populated exercise\
     ;333,adductors,groiners,auto populated exercise\
     ;334,quadriceps,hack squat,auto populated exercise\
     ;335,biceps,hammer curls,auto populated exercise\
     ;336,chest,hammer grip incline db bench press,auto populated exercise\
     ;337,hamstrings,hamstring stretch,auto populated exercise\
     ;338,hamstrings,hamstring-smr,auto populated exercise\
     ;339,shoulders,handstand push-ups,auto populated exercise\
     ;340,quadriceps,hang clean,auto populated exercise\
     ;341,quadriceps,hang clean - below the knees,auto populated exercise\
     ;342,hamstrings,hang snatch,auto populated exercise\
     ;343,hamstrings,hang snatch - below knees,auto populated exercise\
     ;344,hamstrings,hanging bar good morning,auto populated exercise\
     ;345,abdominals,hanging leg raise,auto populated exercise\
     ;346,abdominals,hanging pike,auto populated exercise\
     ;347,quadriceps,heaving snatch balance,auto populated exercise\
     ;348,chest,heavy bag thrust,auto populated exercise\
     ;349,biceps,high cable curls,auto populated exercise\
     ;350,abductors,hip circles (prone),auto populated exercise\
     ;351,glutes,hip extension with bands,auto populated exercise\
     ;352,quadriceps,hip flexion with band,auto populated exercise\
     ;353,glutes,hip lift with band,auto populated exercise\
     ;354,lower back,hug a ball,auto populated exercise\
     ;355,lower back,hug knees to chest,auto populated exercise\
     ;356,hamstrings,hurdle hops,auto populated exercise\
     ;357,lower back,hyperextensions (back extensions),auto populated exercise\
     ;358,lower back,hyperextensions with no hyperextension bench,auto populated exercise\
     ;359,abductors,iliotibial tract-smr,auto populated exercise\
     ;360,triceps,incline barbell triceps extension,auto populated exercise\
     ;361,middle back,incline bench pull,auto populated exercise\
     ;362,chest,incline cable chest press,auto populated exercise\
     ;363,chest,incline cable flye,auto populated exercise\
     ;364,chest,incline dumbbell bench with palms facing in,auto populated exercise\
     ;365,biceps,incline dumbbell curl,auto populated exercise\
     ;366,chest,incline dumbbell flyes,auto populated exercise\
     ;367,chest,incline dumbbell flyes - with a twist,auto populated exercise\
     ;368,chest,incline dumbbell press,auto populated exercise\
     ;369,biceps,incline hammer curls,auto populated exercise\
     ;370,biceps,incline inner biceps curl,auto populated exercise\
     ;371,chest,incline push-up,auto populated exercise\
     ;372,triceps,incline push-up close-grip,auto populated exercise\
     ;373,chest,incline push-up depth jump,auto populated exercise\
     ;374,chest,incline push-up medium,auto populated exercise\
     ;375,chest,incline push-up reverse grip,auto populated exercise\
     ;376,chest,incline push-up wide,auto populated exercise\
     ;377,hamstrings,intermediate groin stretch,auto populated exercise\
     ;378,quadriceps,intermediate hip flexor and quad stretch,auto populated exercise\
     ;379,shoulders,internal rotation with band,auto populated exercise\
     ;380,middle back,inverted row,auto populated exercise\
     ;381,middle back,inverted row with straps,auto populated exercise\
     ;382,shoulders,iron cross,auto populated exercise\
     ;383,quadriceps,iron crosses (stretch),auto populated exercise\
     ;384,chest,isometric chest squeezes,auto populated exercise\
     ;385,neck,isometric neck exercise - front and back,auto populated exercise\
     ;386,neck,isometric neck exercise - sides,auto populated exercise\
     ;387,chest,isometric wipers,auto populated exercise\
     ;388,abductors,it band and glute stretch,auto populated exercise\
     ;389,abdominals,jackknife sit-up,auto populated exercise\
     ;390,abdominals,janda sit-up,auto populated exercise\
     ;391,quadriceps,jefferson squats,auto populated exercise\
     ;392,shoulders,jerk balance,auto populated exercise\
     ;393,quadriceps,jerk dip squat,auto populated exercise\
     ;394,triceps,jm press,auto populated exercise\
     ;395,quadriceps,jogging-treadmill,auto populated exercise\
     ;396,lower back,keg load,auto populated exercise\
     ;397,shoulders,kettlebell arnold press,auto populated exercise\
     ;398,hamstrings,kettlebell dead clean,auto populated exercise\
     ;399,abdominals,kettlebell figure 8,auto populated exercise\
     ;400,hamstrings,kettlebell hang clean,auto populated exercise\
     ;401,hamstrings,kettlebell one-legged deadlift,auto populated exercise\
     ;402,abdominals,kettlebell pass between the legs,auto populated exercise\
     ;403,shoulders,kettlebell pirate ships,auto populated exercise\
     ;404,quadriceps,kettlebell pistol squat,auto populated exercise\
     ;405,shoulders,kettlebell seated press,auto populated exercise\
     ;406,shoulders,kettlebell seesaw press,auto populated exercise\
     ;407,traps,kettlebell sumo high pull,auto populated exercise\
     ;408,shoulders,kettlebell thruster,auto populated exercise\
     ;409,shoulders,kettlebell turkish get-up (lunge style),auto populated exercise\
     ;410,shoulders,kettlebell turkish get-up (squat style),auto populated exercise\
     ;411,abdominals,kettlebell windmill,auto populated exercise\
     ;412,lats,kipping muscle up,auto populated exercise\
     ;413,glutes,knee across the body,auto populated exercise\
     ;414,calves,knee circles,auto populated exercise\
     ;415,hamstrings,knee tuck jump,auto populated exercise\
     ;416,abdominals,knee/hip raise on parallel bars,auto populated exercise\
     ;417,shoulders,kneeling arm drill,auto populated exercise\
     ;418,abdominals,kneeling cable crunch with alternating oblique twists,auto populated exercise\
     ;419,triceps,kneeling cable triceps extension,auto populated exercise\
     ;420,forearms,kneeling forearm stretch,auto populated exercise\
     ;421,lats,kneeling high pulley row,auto populated exercise\
     ;422,quadriceps,kneeling hip flexor,auto populated exercise\
     ;423,glutes,kneeling jump squat,auto populated exercise\
     ;424,lats,kneeling single-arm high pulley row,auto populated exercise\
     ;425,glutes,kneeling squat,auto populated exercise\
     ;426,abdominals,landmine 180's,auto populated exercise\
     ;427,shoulders,landmine linear jammer,auto populated exercise\
     ;428,adductors,lateral bound,auto populated exercise\
     ;429,adductors,lateral box jump,auto populated exercise\
     ;430,adductors,lateral cone hops,auto populated exercise\
     ;431,shoulders,lateral raise - with bands,auto populated exercise\
     ;432,lats,latissimus dorsi-smr,auto populated exercise\
     ;433,quadriceps,leg extensions,auto populated exercise\
     ;434,glutes,leg lift,auto populated exercise\
     ;435,quadriceps,leg press,auto populated exercise\
     ;436,abdominals,leg pull-in,auto populated exercise\
     ;437,chest,leg-over floor press,auto populated exercise\
     ;438,hamstrings,leg-up hamstring stretch,auto populated exercise\
     ;439,chest,leverage chest press,auto populated exercise\
     ;440,quadriceps,leverage deadlift,auto populated exercise\
     ;441,chest,leverage decline chest press,auto populated exercise\
     ;442,middle back,leverage high row,auto populated exercise\
     ;443,chest,leverage incline chest press,auto populated exercise\
     ;444,lats,leverage iso row,auto populated exercise\
     ;445,shoulders,leverage shoulder press,auto populated exercise\
     ;446,traps,leverage shrug,auto populated exercise\
     ;447,hamstrings,linear 3-part start technique,auto populated exercise\
     ;448,hamstrings,linear acceleration wall drill,auto populated exercise\
     ;449,quadriceps,linear depth jump,auto populated exercise\
     ;450,shoulders,log lift,auto populated exercise\
     ;451,lats,london bridges,auto populated exercise\
     ;452,quadriceps,looking at ceiling,auto populated exercise\
     ;453,chest,low cable crossover,auto populated exercise\
     ;454,triceps,low cable triceps extension,auto populated exercise\
     ;455,shoulders,low pulley row to neck,auto populated exercise\
     ;456,abdominals,lower back curl,auto populated exercise\
     ;457,lower back,lower back-smr,auto populated exercise\
     ;458,hamstrings,lunge pass through,auto populated exercise\
     ;459,quadriceps,lunge sprint,auto populated exercise\
     ;460,adductors,lying bent leg groin,auto populated exercise\
     ;461,biceps,lying cable curl,auto populated exercise\
     ;462,middle back,lying cambered barbell row,auto populated exercise\
     ;463,biceps,lying close-grip bar curl on high pulley,auto populated exercise\
     ;464,triceps,lying close-grip barbell triceps extension behind the head,auto populated exercise\
     ;465,triceps,lying close-grip barbell triceps press to chin,auto populated exercise\
     ;466,abductors,lying crossover,auto populated exercise\
     ;467,triceps,lying dumbbell tricep extension,auto populated exercise\
     ;468,neck,lying face down plate neck resistance,auto populated exercise\
     ;469,neck,lying face up plate neck resistance,auto populated exercise\
     ;470,glutes,lying glute,auto populated exercise\
     ;471,hamstrings,lying hamstring,auto populated exercise\
     ;472,biceps,lying high bench barbell curl,auto populated exercise\
     ;473,hamstrings,lying leg curls,auto populated exercise\
     ;474,quadriceps,lying machine squat,auto populated exercise\
     ;475,shoulders,lying one-arm lateral raise,auto populated exercise\
     ;476,quadriceps,lying prone quadriceps,auto populated exercise\
     ;477,shoulders,lying rear delt raise,auto populated exercise\
     ;478,biceps,lying supine dumbbell curl,auto populated exercise\
     ;479,middle back,lying t-bar row,auto populated exercise\
     ;480,triceps,lying triceps press,auto populated exercise\
     ;481,chest,machine bench press,auto populated exercise\
     ;482,biceps,machine bicep curl,auto populated exercise\
     ;483,biceps,machine preacher curls,auto populated exercise\
     ;484,shoulders,machine shoulder (military) press,auto populated exercise\
     ;485,triceps,machine triceps extension,auto populated exercise\
     ;486,chest,medicine ball chest pass,auto populated exercise\
     ;487,abdominals,medicine ball full twist,auto populated exercise\
     ;488,shoulders,medicine ball scoop throw,auto populated exercise\
     ;489,middle back,middle back shrug,auto populated exercise\
     ;490,middle back,middle back stretch,auto populated exercise\
     ;491,middle back,mixed grip chin,auto populated exercise\
     ;492,abductors,monster walk,auto populated exercise\
     ;493,quadriceps,mountain climbers,auto populated exercise\
     ;494,hamstrings,moving claw series,auto populated exercise\
     ;495,hamstrings,muscle snatch,auto populated exercise\
     ;496,lats,muscle up,auto populated exercise\
     ;497,quadriceps,narrow stance hack squats,auto populated exercise\
     ;498,quadriceps,narrow stance leg press,auto populated exercise\
     ;499,quadriceps,narrow stance squats,auto populated exercise\
     ;500,hamstrings,natural glute ham raise,auto populated exercise\
     ;501,chest,neck press,auto populated exercise\
     ;502,neck,neck-smr,auto populated exercise\
     ;503,abdominals,oblique crunches,auto populated exercise\
     ;504,abdominals,oblique crunches - on the floor,auto populated exercise\
     ;505,quadriceps,olympic squat,auto populated exercise\
     ;506,quadriceps,on your side quad stretch,auto populated exercise\
     ;507,quadriceps,on-your-back quad stretch,auto populated exercise\
     ;508,lats,one arm against wall,auto populated exercise\
     ;509,middle back,one arm chin-up,auto populated exercise\
     ;510,chest,one arm dumbbell bench press,auto populated exercise\
     ;511,biceps,one arm dumbbell preacher curl,auto populated exercise\
     ;512,triceps,one arm floor press,auto populated exercise\
     ;513,lats,one arm lat pulldown,auto populated exercise\
     ;514,triceps,one arm pronated dumbbell triceps extension,auto populated exercise\
     ;515,triceps,one arm supinated dumbbell triceps extension,auto populated exercise\
     ;516,quadriceps,one half locust,auto populated exercise\
     ;517,lats,one handed hang,auto populated exercise\
     ;518,glutes,one knee to chest,auto populated exercise\
     ;519,quadriceps,one leg barbell squat,auto populated exercise\
     ;520,middle back,one-arm dumbbell row,auto populated exercise\
     ;521,chest,one-arm flat bench dumbbell flye,auto populated exercise\
     ;522,abdominals,one-arm high-pulley cable side bends,auto populated exercise\
     ;523,shoulders,one-arm incline lateral raise,auto populated exercise\
     ;524,hamstrings,one-arm kettlebell clean,auto populated exercise\
     ;525,shoulders,one-arm kettlebell clean and jerk,auto populated exercise\
     ;526,chest,one-arm kettlebell floor press,auto populated exercise\
     ;527,shoulders,one-arm kettlebell jerk,auto populated exercise\
     ;528,shoulders,one-arm kettlebell military press to the side,auto populated exercise\
     ;529,shoulders,one-arm kettlebell para press,auto populated exercise\
     ;530,shoulders,one-arm kettlebell push press,auto populated exercise\
     ;531,middle back,one-arm kettlebell row,auto populated exercise\
     ;532,shoulders,one-arm kettlebell snatch,auto populated exercise\
     ;533,shoulders,one-arm kettlebell split jerk,auto populated exercise\
     ;534,shoulders,one-arm kettlebell split snatch,auto populated exercise\
     ;535,hamstrings,one-arm kettlebell swings,auto populated exercise\
     ;536,middle back,one-arm long bar row,auto populated exercise\
     ;537,abdominals,one-arm medicine ball slam,auto populated exercise\
     ;538,hamstrings,one-arm open palm kettlebell clean,auto populated exercise\
     ;539,quadriceps,one-arm overhead kettlebell squats,auto populated exercise\
     ;540,quadriceps,one-arm side deadlift,auto populated exercise\
     ;541,shoulders,one-arm side laterals,auto populated exercise\
     ;542,glutes,one-legged cable kickback,auto populated exercise\
     ;543,hamstrings,open palm kettlebell clean,auto populated exercise\
     ;544,abdominals,otis-up,auto populated exercise\
     ;545,biceps,overhead cable curl,auto populated exercise\
     ;546,lats,overhead lat,auto populated exercise\
     ;547,lats,overhead slam,auto populated exercise\
     ;548,quadriceps,overhead squat,auto populated exercise\
     ;549,abdominals,overhead stretch,auto populated exercise\
     ;550,triceps,overhead triceps,auto populated exercise\
     ;551,abdominals,pallof press with rotation,auto populated exercise\
     ;552,forearms,palms-down dumbbell wrist curl over a bench,auto populated exercise\
     ;553,forearms,palms-down wrist curl over a bench,auto populated exercise\
     ;554,forearms,palms-up barbell wrist curl over a bench,auto populated exercise\
     ;555,forearms,palms-up dumbbell wrist curl over a bench,auto populated exercise\
     ;556,triceps,parallel bar dip,auto populated exercise\
     ;557,lower back,pelvic tilt into bridge,auto populated exercise\
     ;558,calves,peroneals stretch,auto populated exercise\
     ;559,calves,peroneals-smr,auto populated exercise\
     ;560,glutes,physioball hip bridge,auto populated exercise\
     ;561,triceps,pin presses,auto populated exercise\
     ;562,glutes,piriformis-smr,auto populated exercise\
     ;563,abdominals,plank,auto populated exercise\
     ;564,abdominals,plank with twist,auto populated exercise\
     ;565,forearms,plate pinch,auto populated exercise\
     ;566,abdominals,plate twist,auto populated exercise\
     ;567,hamstrings,platform hamstring slides,auto populated exercise\
     ;568,quadriceps,plie dumbbell squat,auto populated exercise\
     ;569,chest,plyo kettlebell pushups,auto populated exercise\
     ;570,chest,plyo push-up,auto populated exercise\
     ;571,calves,posterior tibialis stretch,auto populated exercise\
     ;572,hamstrings,power clean,auto populated exercise\
     ;573,hamstrings,power clean from blocks,auto populated exercise\
     ;574,quadriceps,power jerk,auto populated exercise\
     ;575,shoulders,power partials,auto populated exercise\
     ;576,hamstrings,power snatch,auto populated exercise\
     ;577,quadriceps,power snatch from blocks,auto populated exercise\
     ;578,hamstrings,power stairs,auto populated exercise\
     ;579,biceps,preacher curl,auto populated exercise\
     ;580,biceps,preacher hammer dumbbell curl,auto populated exercise\
     ;581,abdominals,press sit-up,auto populated exercise\
     ;582,hamstrings,prone manual hamstring,auto populated exercise\
     ;583,hamstrings,prowler sprint,auto populated exercise\
     ;584,glutes,pull through,auto populated exercise\
     ;585,lats,pullups,auto populated exercise\
     ;586,shoulders,push press,auto populated exercise\
     ;587,shoulders,push press - behind the neck,auto populated exercise\
     ;588,chest,push up to side plank,auto populated exercise\
     ;589,chest,push-up wide,auto populated exercise\
     ;590,triceps,push-ups - close triceps position,auto populated exercise\
     ;591,chest,push-ups with feet elevated,auto populated exercise\
     ;592,chest,push-ups with feet on an exercise ball,auto populated exercise\
     ;593,chest,pushups,auto populated exercise\
     ;594,chest,pushups (close and wide hand positions),auto populated exercise\
     ;595,lower back,pyramid,auto populated exercise\
     ;596,quadriceps,quad stretch,auto populated exercise\
     ;597,quadriceps,quadriceps-smr,auto populated exercise\
     ;598,quadriceps,quick leap,auto populated exercise\
     ;599,shoulders,rack delivery,auto populated exercise\
     ;600,lower back,rack pull with bands,auto populated exercise\
     ;601,lower back,rack pulls,auto populated exercise\
     ;602,quadriceps,rear leg raises,auto populated exercise\
     ;603,quadriceps,recumbent bike,auto populated exercise\
     ;604,shoulders,return push from stance,auto populated exercise\
     ;605,triceps,reverse band bench press,auto populated exercise\
     ;606,quadriceps,reverse band box squat,auto populated exercise\
     ;607,lower back,reverse band deadlift,auto populated exercise\
     ;608,quadriceps,reverse band power squat,auto populated exercise\
     ;609,hamstrings,reverse band sumo deadlift,auto populated exercise\
     ;610,biceps,reverse barbell curl,auto populated exercise\
     ;611,biceps,reverse barbell preacher curls,auto populated exercise\
     ;612,biceps,reverse cable curl,auto populated exercise\
     ;613,abdominals,reverse crunch,auto populated exercise\
     ;614,shoulders,reverse flyes,auto populated exercise\
     ;615,shoulders,reverse flyes with external rotation,auto populated exercise\
     ;616,middle back,reverse grip bent-over rows,auto populated exercise\
     ;617,triceps,reverse grip triceps pushdown,auto populated exercise\
     ;618,hamstrings,reverse hyperextension,auto populated exercise\
     ;619,shoulders,reverse machine flyes,auto populated exercise\
     ;620,biceps,reverse plate curls,auto populated exercise\
     ;621,triceps,reverse triceps bench press,auto populated exercise\
     ;622,middle back,rhomboids-smr,auto populated exercise\
     ;623,forearms,rickshaw carry,auto populated exercise\
     ;624,quadriceps,rickshaw deadlift,auto populated exercise\
     ;625,triceps,ring dips,auto populated exercise\
     ;626,quadriceps,rocket jump,auto populated exercise\
     ;627,calves,rocking standing calf raise,auto populated exercise\
     ;628,lats,rocky pull-ups/pulldowns,auto populated exercise\
     ;629,hamstrings,romanian deadlift,auto populated exercise\
     ;630,hamstrings,romanian deadlift from deficit,auto populated exercise\
     ;631,lats,rope climb,auto populated exercise\
     ;632,abdominals,rope crunch,auto populated exercise\
     ;633,quadriceps,rope jumping,auto populated exercise\
     ;634,lats,rope straight-arm pulldown,auto populated exercise\
     ;635,shoulders,round the world shoulder stretch,auto populated exercise\
     ;636,quadriceps,rowing stationary,auto populated exercise\
     ;637,hamstrings,runner's stretch,auto populated exercise\
     ;638,quadriceps,running treadmill,auto populated exercise\
     ;639,abdominals,russian twist,auto populated exercise\
     ;640,quadriceps,sandbag load,auto populated exercise\
     ;641,traps,scapular pull-up,auto populated exercise\
     ;642,abdominals,scissor kick,auto populated exercise\
     ;643,quadriceps,scissors jump,auto populated exercise\
     ;644,hamstrings,seated band hamstring curl,auto populated exercise\
     ;645,shoulders,seated barbell military press,auto populated exercise\
     ;646,abdominals,seated barbell twist,auto populated exercise\
     ;647,triceps,seated bent-over one-arm dumbbell triceps extension,auto populated exercise\
     ;648,shoulders,seated bent-over rear delt raise,auto populated exercise\
     ;649,triceps,seated bent-over two-arm dumbbell triceps extension,auto populated exercise\
     ;650,biceps,seated biceps,auto populated exercise\
     ;651,middle back,seated cable rows,auto populated exercise\
     ;652,shoulders,seated cable shoulder press,auto populated exercise\
     ;653,calves,seated calf raise,auto populated exercise\
     ;654,calves,seated calf stretch,auto populated exercise\
     ;655,biceps,seated close-grip concentration barbell curl,auto populated exercise\
     ;656,biceps,seated dumbbell curl,auto populated exercise\
     ;657,biceps,seated dumbbell inner biceps curl,auto populated exercise\
     ;658,forearms,seated dumbbell palms-down wrist curl,auto populated exercise\
     ;659,forearms,seated dumbbell palms-up wrist curl,auto populated exercise\
     ;660,shoulders,seated dumbbell press,auto populated exercise\
     ;661,abdominals,seated flat bench leg pull-in,auto populated exercise\
     ;662,hamstrings,seated floor hamstring stretch,auto populated exercise\
     ;663,shoulders,seated front deltoid,auto populated exercise\
     ;664,glutes,seated glute,auto populated exercise\
     ;665,lower back,seated good mornings,auto populated exercise\
     ;666,hamstrings,seated hamstring,auto populated exercise\
     ;667,hamstrings,seated hamstring and calf stretch,auto populated exercise\
     ;668,neck,seated head harness neck resistance,auto populated exercise\
     ;669,hamstrings,seated leg curl,auto populated exercise\
     ;670,abdominals,seated leg tucks,auto populated exercise\
     ;671,middle back,seated one-arm cable pulley rows,auto populated exercise\
     ;672,forearms,seated one-arm dumbbell palms-down wrist curl,auto populated exercise\
     ;673,forearms,seated one-arm dumbbell palms-up wrist curl,auto populated exercise\
     ;674,abdominals,seated overhead stretch,auto populated exercise\
     ;675,forearms,seated palm-up barbell wrist curl,auto populated exercise\
     ;676,forearms,seated palms-down barbell wrist curl,auto populated exercise\
     ;677,shoulders,seated side lateral raise,auto populated exercise\
     ;678,triceps,seated triceps press,auto populated exercise\
     ;679,forearms,seated two-arm palms-up low-pulley wrist curl,auto populated exercise\
     ;680,shoulders,see-saw press (alternating side press),auto populated exercise\
     ;681,lats,shotgun row,auto populated exercise\
     ;682,shoulders,shoulder circles,auto populated exercise\
     ;683,shoulders,shoulder press - with bands,auto populated exercise\
     ;684,shoulders,shoulder raise,auto populated exercise\
     ;685,shoulders,shoulder stretch,auto populated exercise\
     ;686,abdominals,side bridge,auto populated exercise\
     ;687,quadriceps,side hop-sprint,auto populated exercise\
     ;688,abdominals,side jackknife,auto populated exercise\
     ;689,shoulders,side lateral raise,auto populated exercise\
     ;690,shoulders,side laterals to front raise,auto populated exercise\
     ;691,adductors,side leg raises,auto populated exercise\
     ;692,adductors,side lying groin stretch,auto populated exercise\
     ;693,neck,side neck stretch,auto populated exercise\
     ;694,quadriceps,side standing long jump,auto populated exercise\
     ;695,quadriceps,side to side box shuffle,auto populated exercise\
     ;696,lats,side to side chins,auto populated exercise\
     ;697,shoulders,side wrist pull,auto populated exercise\
     ;698,lats,side-lying floor stretch,auto populated exercise\
     ;699,shoulders,single dumbbell raise,auto populated exercise\
     ;700,quadriceps,single leg butt kick,auto populated exercise\
     ;701,glutes,single leg glute bridge,auto populated exercise\
     ;702,quadriceps,single leg push-off,auto populated exercise\
     ;703,chest,single-arm cable crossover,auto populated exercise\
     ;704,shoulders,single-arm linear jammer,auto populated exercise\
     ;705,chest,single-arm push-up,auto populated exercise\
     ;706,quadriceps,single-cone sprint drill,auto populated exercise\
     ;707,quadriceps,single-leg high box squat,auto populated exercise\
     ;708,quadriceps,single-leg hop progression,auto populated exercise\
     ;709,quadriceps,single-leg lateral hop,auto populated exercise\
     ;710,quadriceps,single-leg leg extension,auto populated exercise\
     ;711,quadriceps,single-leg stride jump,auto populated exercise\
     ;712,quadriceps,sit squats,auto populated exercise\
     ;713,abdominals,sit-up,auto populated exercise\
     ;714,quadriceps,skating,auto populated exercise\
     ;715,quadriceps,sled drag - harness,auto populated exercise\
     ;716,shoulders,sled overhead backward walk,auto populated exercise\
     ;717,triceps,sled overhead triceps extension,auto populated exercise\
     ;718,quadriceps,sled push,auto populated exercise\
     ;719,shoulders,sled reverse flye,auto populated exercise\
     ;720,middle back,sled row,auto populated exercise\
     ;721,abdominals,sledgehammer swings,auto populated exercise\
     ;722,shoulders,smith incline shoulder raise,auto populated exercise\
     ;723,traps,smith machine behind the back shrug,auto populated exercise\
     ;724,chest,smith machine bench press,auto populated exercise\
     ;725,middle back,smith machine bent over row,auto populated exercise\
     ;726,calves,smith machine calf raise,auto populated exercise\
     ;727,triceps,smith machine close-grip bench press,auto populated exercise\
     ;728,chest,smith machine decline press,auto populated exercise\
     ;729,hamstrings,smith machine hang power clean,auto populated exercise\
     ;730,abdominals,smith machine hip raise,auto populated exercise\
     ;731,chest,smith machine incline bench press,auto populated exercise\
     ;732,quadriceps,smith machine leg press,auto populated exercise\
     ;733,shoulders,smith machine one-arm upright row,auto populated exercise\
     ;734,shoulders,smith machine overhead shoulder press,auto populated exercise\
     ;735,quadriceps,smith machine pistol squat,auto populated exercise\
     ;736,calves,smith machine reverse calf raises,auto populated exercise\
     ;737,traps,smith machine shrug,auto populated exercise\
     ;738,quadriceps,smith machine squat,auto populated exercise\
     ;739,hamstrings,smith machine stiff-legged deadlift,auto populated exercise\
     ;740,traps,smith machine upright row,auto populated exercise\
     ;741,quadriceps,smith single-leg split squat,auto populated exercise\
     ;742,quadriceps,snatch,auto populated exercise\
     ;743,quadriceps,snatch balance,auto populated exercise\
     ;744,hamstrings,snatch deadlift,auto populated exercise\
     ;745,quadriceps,snatch from blocks,auto populated exercise\
     ;746,hamstrings,snatch pull,auto populated exercise\
     ;747,traps,snatch shrug,auto populated exercise\
     ;748,triceps,speed band overhead triceps,auto populated exercise\
     ;749,triceps,speed band pushdown,auto populated exercise\
     ;750,quadriceps,speed box squat,auto populated exercise\
     ;751,quadriceps,speed squats,auto populated exercise\
     ;752,abdominals,spell caster,auto populated exercise\
     ;753,abdominals,spider crawl,auto populated exercise\
     ;754,biceps,spider curl,auto populated exercise\
     ;755,middle back,spinal stretch,auto populated exercise\
     ;756,quadriceps,split clean,auto populated exercise\
     ;757,quadriceps,split jerk,auto populated exercise\
     ;758,quadriceps,split jump,auto populated exercise\
     ;759,hamstrings,split snatch,auto populated exercise\
     ;760,quadriceps,split squat with dumbbells,auto populated exercise\
     ;761,hamstrings,split squats,auto populated exercise\
     ;762,quadriceps,squat jerk,auto populated exercise\
     ;763,quadriceps,squat with bands,auto populated exercise\
     ;764,quadriceps,squat with chains,auto populated exercise\
     ;765,quadriceps,squat with plate movers,auto populated exercise\
     ;766,quadriceps,squats - with bands,auto populated exercise\
     ;767,quadriceps,stairmaster,auto populated exercise\
     ;768,shoulders,standing alternating dumbbell press,auto populated exercise\
     ;769,calves,standing barbell calf raise,auto populated exercise\
     ;770,shoulders,standing barbell press behind neck,auto populated exercise\
     ;771,triceps,standing bent-over one-arm dumbbell triceps extension,auto populated exercise\
     ;772,triceps,standing bent-over two-arm dumbbell triceps extension,auto populated exercise\
     ;773,biceps,standing biceps cable curl,auto populated exercise\
     ;774,biceps,standing biceps stretch,auto populated exercise\
     ;775,shoulders,standing bradford press,auto populated exercise\
     ;776,chest,standing cable chest press,auto populated exercise\
     ;777,abdominals,standing cable lift,auto populated exercise\
     ;778,abdominals,standing cable wood chop,auto populated exercise\
     ;779,calves,standing calf raises,auto populated exercise\
     ;780,biceps,standing concentration curl,auto populated exercise\
     ;781,calves,standing dumbbell calf raise,auto populated exercise\
     ;782,shoulders,standing dumbbell press,auto populated exercise\
     ;783,biceps,standing dumbbell reverse curl,auto populated exercise\
     ;784,shoulders,standing dumbbell straight-arm front delt raise above head,auto populated exercise\
     ;785,triceps,standing dumbbell triceps extension,auto populated exercise\
     ;786,traps,standing dumbbell upright row,auto populated exercise\
     ;787,quadriceps,standing elevated quad stretch,auto populated exercise\
     ;788,shoulders,standing front barbell raise over head,auto populated exercise\
     ;789,calves,standing gastrocnemius calf stretch,auto populated exercise\
     ;790,hamstrings,standing hamstring and calf stretch,auto populated exercise\
     ;791,abductors,standing hip circles,auto populated exercise\
     ;792,quadriceps,standing hip flexors,auto populated exercise\
     ;793,biceps,standing inner-biceps curl,auto populated exercise\
     ;794,abdominals,standing lateral stretch,auto populated exercise\
     ;795,hamstrings,standing leg curl,auto populated exercise\
     ;796,quadriceps,standing long jump,auto populated exercise\
     ;797,shoulders,standing low-pulley deltoid raise,auto populated exercise\
     ;798,triceps,standing low-pulley one-arm triceps extension,auto populated exercise\
     ;799,shoulders,standing military press,auto populated exercise\
     ;800,forearms,standing olympic plate hand squeeze,auto populated exercise\
     ;801,biceps,standing one-arm cable curl,auto populated exercise\
     ;802,biceps,standing one-arm dumbbell curl over incline bench,auto populated exercise\
     ;803,triceps,standing one-arm dumbbell triceps extension,auto populated exercise\
     ;804,triceps,standing overhead barbell triceps extension,auto populated exercise\
     ;805,shoulders,standing palm-in one-arm dumbbell press,auto populated exercise\
     ;806,shoulders,standing palms-in dumbbell press,auto populated exercise\
     ;807,forearms,standing palms-up barbell behind the back wrist curl,auto populated exercise\
     ;808,lower back,standing pelvic tilt,auto populated exercise\
     ;809,abdominals,standing rope crunch,auto populated exercise\
     ;810,calves,standing soleus and achilles stretch,auto populated exercise\
     ;811,hamstrings,standing toe touches,auto populated exercise\
     ;812,triceps,standing towel triceps extension,auto populated exercise\
     ;813,shoulders,standing two-arm overhead throw,auto populated exercise\
     ;814,quadriceps,star jump,auto populated exercise\
     ;815,quadriceps,step mill,auto populated exercise\
     ;816,glutes,step-up with knee raise,auto populated exercise\
     ;817,lower back,stiff leg barbell good morning,auto populated exercise\
     ;818,hamstrings,stiff-legged barbell deadlift,auto populated exercise\
     ;819,hamstrings,stiff-legged dumbbell deadlift,auto populated exercise\
     ;820,abdominals,stomach vacuum,auto populated exercise\
     ;821,middle back,straight bar bench mid rows,auto populated exercise\
     ;822,shoulders,straight raises on incline bench,auto populated exercise\
     ;823,chest,straight-arm dumbbell pullover,auto populated exercise\
     ;824,lats,straight-arm pulldown,auto populated exercise\
     ;825,quadriceps,stride jump crossover,auto populated exercise\
     ;826,hamstrings,sumo deadlift,auto populated exercise\
     ;827,hamstrings,sumo deadlift with bands,auto populated exercise\
     ;828,hamstrings,sumo deadlift with chains,auto populated exercise\
     ;829,lower back,superman,auto populated exercise\
     ;830,triceps,supine chest throw,auto populated exercise\
     ;831,abdominals,supine one-arm overhead throw,auto populated exercise\
     ;832,abdominals,supine two-arm overhead throw,auto populated exercise\
     ;833,chest,suspended push-up,auto populated exercise\
     ;834,abdominals,suspended reverse crunch,auto populated exercise\
     ;835,middle back,suspended row,auto populated exercise\
     ;836,quadriceps,suspended split squat,auto populated exercise\
     ;837,middle back,t-bar row with handle,auto populated exercise\
     ;838,triceps,tate press,auto populated exercise\
     ;839,hamstrings,the straddle,auto populated exercise\
     ;840,abductors,thigh abductor,auto populated exercise\
     ;841,adductors,thigh adductor,auto populated exercise\
     ;842,quadriceps,tire flip,auto populated exercise\
     ;843,abdominals,toe touchers,auto populated exercise\
     ;844,abdominals,torso rotation,auto populated exercise\
     ;845,quadriceps,trail running/walking,auto populated exercise\
     ;846,quadriceps,trap bar deadlift,auto populated exercise\
     ;847,triceps,tricep dumbbell kickback,auto populated exercise\
     ;848,triceps,tricep side stretch,auto populated exercise\
     ;849,triceps,triceps overhead extension with rope,auto populated exercise\
     ;850,triceps,triceps pushdown,auto populated exercise\
     ;851,triceps,triceps pushdown - rope attachment,auto populated exercise\
     ;852,triceps,triceps pushdown - v-bar attachment,auto populated exercise\
     ;853,triceps,triceps stretch,auto populated exercise\
     ;854,abdominals,tuck crunch,auto populated exercise\
     ;855,biceps,two-arm dumbbell preacher curl,auto populated exercise\
     ;856,shoulders,two-arm kettlebell clean,auto populated exercise\
     ;857,shoulders,two-arm kettlebell jerk,auto populated exercise\
     ;858,shoulders,two-arm kettlebell military press,auto populated exercise\
     ;859,middle back,two-arm kettlebell row,auto populated exercise\
     ;860,lats,underhand cable pulldowns,auto populated exercise\
     ;861,middle back,upper back stretch,auto populated exercise\
     ;862,hamstrings,upper back-leg grab,auto populated exercise\
     ;863,shoulders,upright barbell row,auto populated exercise\
     ;864,traps,upright cable row,auto populated exercise\
     ;865,traps,upright row - with bands,auto populated exercise\
     ;866,shoulders,upward stretch,auto populated exercise\
     ;867,lats,v-bar pulldown,auto populated exercise\
     ;868,lats,v-bar pullup,auto populated exercise\
     ;869,hamstrings,vertical swing,auto populated exercise\
     ;870,quadriceps,walking treadmill,auto populated exercise\
     ;871,lower back,weighted ball hyperextension,auto populated exercise\
     ;872,abdominals,weighted ball side bend,auto populated exercise\
     ;873,triceps,weighted bench dip,auto populated exercise\
     ;874,abdominals,weighted crunches,auto populated exercise\
     ;875,quadriceps,weighted jump squat,auto populated exercise\
     ;876,lats,weighted pull ups,auto populated exercise\
     ;877,quadriceps,weighted sissy squat,auto populated exercise\
     ;878,abdominals,weighted sit-ups - with bands,auto populated exercise\
     ;879,quadriceps,weighted squat,auto populated exercise\
     ;880,quadriceps,wide stance barbell squat,auto populated exercise\
     ;881,hamstrings,wide stance stiff legs,auto populated exercise\
     ;882,chest,wide-grip barbell bench press,auto populated exercise\
     ;883,chest,wide-grip decline barbell bench press,auto populated exercise\
     ;884,chest,wide-grip decline barbell pullover,auto populated exercise\
     ;885,lats,wide-grip lat pulldown,auto populated exercise\
     ;886,lats,wide-grip pulldown behind the neck,auto populated exercise\
     ;887,lats,wide-grip rear pull-up,auto populated exercise\
     ;888,biceps,wide-grip standing barbell curl,auto populated exercise\
     ;889,abdominals,wind sprints,auto populated exercise\
     ;890,abductors,windmills,auto populated exercise\
     ;891,hamstrings,world's greatest stretch,auto populated exercise\
     ;892,forearms,wrist circles,auto populated exercise\
     ;893,forearms,wrist roller,auto populated exercise\
     ;894,forearms,wrist rotations with straight bar,auto populated exercise\
     ;895,quadriceps,yoke walk,auto populated exercise\
     ;896,quadriceps,zercher squats,auto populated exercise\
     ;897,biceps,zottman curl,auto populated exercise\
     ;898,biceps,zottman preacher curl,auto populated exercise";

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
