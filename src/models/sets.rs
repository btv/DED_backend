use serde::{Serialize, Deserialize};
use std::time::SystemTime;

use crate::schema::sets;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
pub struct Set {
    pub id: i32,
    pub exercise_id: i32,
    pub style: String,
    pub unit: String,
    pub goal_reps: i16,
    pub goal_value: String,
    pub description: String,
    pub created_or_completed: SystemTime,
    pub completed_reps: i16,
    pub completed_value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "sets"]
pub struct NewSet {
    pub id: i32,
    pub exercise_id: i32,
    pub style: String,
    pub unit: String,
    pub goal_reps: i16,
    pub goal_value: String,
    pub description: String,
    pub created_or_completed: SystemTime,
    pub completed_reps: i16,
    pub completed_value: String,
}

impl NewSet {
    pub fn create(&self) -> Result<Set, diesel::result::Error> {
        use diesel::RunQueryDsl;
        use crate::establish_connection;

        let connection = establish_connection();
        diesel::insert_into(sets::table)
            .values(self)
            .get_result(&connection)
    }
}

#[cfg(test)]
mod tests {
    use std::time::SystemTime;
    use crate::models::sets::Set;

    #[test]
    fn test_new_set(){
        let t_id = 10;
        let t_exercise_id = 100;
        let t_style = "Fancy";
        let t_unit = "steps";
        let t_goal_reps = 45;
        let t_goal_value = "To survive";
        let t_description = "Twist and shout";
        let t_created_or_completed = SystemTime::now();
        let t_completed_reps = 10;
        let t_completed_value = "Should this be a string";

        let t_set = Set{
            id: t_id,
            exercise_id: t_exercise_id,
            style: t_style.to_string(),
            unit: t_unit.to_string(),
            goal_reps: t_goal_reps,
            goal_value: t_goal_value.to_string(),
            description:t_description.to_string(),
            created_or_completed: t_created_or_completed,
            completed_reps: t_completed_reps,
            completed_value: t_completed_value.to_string()
        };

        assert_eq!(t_id, t_set.id);
        assert_eq!(t_exercise_id, t_set.exercise_id);
        assert_eq!(t_style, t_set.style);
        assert_eq!(t_unit, t_set.unit);
        assert_eq!(t_goal_reps, t_set.goal_reps);
        assert_eq!(t_description, t_set.description);
        assert_eq!(t_created_or_completed, t_set.created_or_completed);
        assert_eq!(t_completed_reps, t_set.completed_reps);
        assert_eq!(t_completed_value, t_set.completed_value);




    }
}