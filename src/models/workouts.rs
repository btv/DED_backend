use serde::{Serialize, Deserialize};
use std::time::SystemTime;

use crate::schema::workouts;
use diesel::RunQueryDsl;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
pub struct Workout {
    pub id: i32,
    pub origin_id: i32,
    pub exercise: i32,
    pub fname: String,
    pub description: String,
    pub notes: String,
    pub created_time: SystemTime,
    pub completed_time: SystemTime,
    pub create_id: i32,
    pub completed_id: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[table_name="workouts"]
pub struct NewWorkout {
    pub id: i32,
    pub origin_id: i32,
    pub exercise: i32,
    pub fname: String,
    pub description: String,
    pub notes: String,
    pub created_time: SystemTime,
    pub completed_time: SystemTime,
    pub create_id: i32,
    pub completed_id: i32,
}

impl NewWorkout{
    pub fn create(&self)->Result<Workout, diesel::result::Error>
    {
        use crate::establish_connection;
        let conn = establish_connection();
        diesel::insert_into(workouts::table)
            .values(self)
            .get_result(&conn)
    }
}

#[cfg(test)]
mod tests {
    use std::time::{SystemTime, Duration};
    use crate::models::workouts::Workout;

    #[test]
    fn test_new_workout_structure(){

        let t_id = -1;
        let t_origin_id = 100;
        let t_exercise = 10;
        let t_fname = "Test Workout";
        let t_description = "Quads and calves";
        let t_notes = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.";
        let t_created_time = SystemTime::now();
        let t_completed_time = t_created_time + Duration::new(300,0);
        let t_create_id = 100;
        let t_completed_id = 10;


        let t_workout = Workout{
            id : t_id,
            origin_id: t_origin_id,
            exercise: t_exercise,
            fname: t_fname.to_string(),
            description: t_description.to_string(),
            notes: t_notes.to_string(),
            created_time: t_created_time,
            completed_time: t_completed_time,
            create_id: t_create_id,
            completed_id: t_completed_id
        };

        assert_eq!(t_id, t_workout.id);
        assert_eq!(t_origin_id, t_workout.origin_id);
        assert_eq!(t_exercise, t_workout.exercise);
        assert_eq!(t_fname, t_workout.fname);
        assert_eq!(t_description, t_workout.description);
        assert_eq!(t_notes, t_workout.notes);
        assert_eq!(t_created_time, t_workout.created_time);
        assert_eq!(t_create_id, t_workout.create_id);
        assert_eq!(t_completed_id, t_workout.completed_id);
    }
}
