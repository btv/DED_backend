use serde::{Serialize, Deserialize};
use std::time::SystemTime;

use crate::schema::workouts;
use diesel::{PgConnection,RunQueryDsl};
use diesel::query_dsl::filter_dsl::FindDsl;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
pub struct Workout {
    pub id: i32,
    pub origin_id: i32,
    pub exercise: i32,
    pub name: String,
    pub description: String,
    pub notes: String,
    pub created_time: SystemTime,
    pub completed_time: SystemTime,
    pub create_id: i32,
    pub completed_id: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable, AsChangeset)]
#[table_name="workouts"]
pub struct NewWorkout {
    pub origin_id: i32,
    pub exercise: i32,
    pub name: String,
    pub description: String,
    pub notes: String,
}

impl NewWorkout{
    pub fn create(&self, conn: &PgConnection)->Result<Workout, diesel::result::Error>
    {
        diesel::insert_into(workouts::table)
            .values(self)
            .get_result(conn)
    }
}


impl Workout{
    pub fn get_workout_by_id(id:i32, conn: &PgConnection) ->Result<Workout, diesel::result::Error>{
       workouts::table.find(id).get_result(conn)
    }

    pub fn delete(in_id: i32, connection: &PgConnection) -> Result<usize, diesel::result::Error> {
        diesel::delete(workouts::table.find(in_id)).execute(connection)
    }

    pub fn update(in_id: i32, new_set: &NewWorkout, connection: &PgConnection) -> Result<usize, diesel::result::Error> {

        diesel::update(workouts::table.find(in_id))
            .set(new_set)
            .execute(connection)
    }

}

impl PartialEq<NewWorkout> for Workout {
    fn eq(&self, other:& NewWorkout) -> bool {
        self.origin_id == other.origin_id &&
        self.exercise == other.exercise &&
        self.name == other.name &&
        self.description == other.description &&
        self.notes == other.notes
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
        let t_name = "Test Workout";
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
            name: t_name.to_string(),
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
        assert_eq!(t_name, t_workout.name);
        assert_eq!(t_description, t_workout.description);
        assert_eq!(t_notes, t_workout.notes);
        assert_eq!(t_created_time, t_workout.created_time);
        assert_eq!(t_create_id, t_workout.create_id);
        assert_eq!(t_completed_id, t_workout.completed_id);
    }
}
