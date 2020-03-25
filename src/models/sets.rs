use serde::{Serialize, Deserialize};
use std::time::SystemTime;

use crate::schema::sets;
use diesel::query_dsl::filter_dsl::FindDsl;
use diesel::PgConnection;

#[derive(
    PartialEq, Eq, Debug, Clone, Queryable, Identifiable, Insertable,
    AsChangeset, QueryableByName, Serialize, Deserialize
)]
#[table_name = "sets"]
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

#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[table_name = "sets"]
pub struct NewSet {
    pub exercise_id:i32,
    pub style: String,
    pub unit: String,
    pub goal_reps: i16,
    pub goal_value: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetList( pub Vec<Set> );

impl NewSet {
    pub fn create(&self, connection: &PgConnection) -> Result<Set, diesel::result::Error> {
        use diesel::RunQueryDsl;

        diesel::insert_into(sets::table)
            .values(self)
            .get_result(connection)
    }
}

impl Set {
    pub fn get_set_by_id(id: i32, conn: &PgConnection) -> Result<Set, diesel::result::Error> {
        use diesel::RunQueryDsl;

        sets::table.find(id).get_result(conn)
    }

    pub fn delete(in_id: &i32, connection: &PgConnection) -> Result<usize, diesel::result::Error> {
        use diesel::RunQueryDsl;

        diesel::delete(sets::table.find(*in_id)).execute(connection)
    }
}

impl SetList {
    pub fn get_sets_by_exercise_id(ex_id: &i32, conn: &PgConnection) -> Self {
        use diesel::prelude::*;
        use crate::schema::sets::dsl::exercise_id;

        let results = sets::table.filter(exercise_id.eq(*ex_id))
            .load::<Set>(conn)
            .expect("Error loading sets");

        SetList(results)
    }
}


#[cfg(test)]
mod tests {
    use std::time::SystemTime;
    use crate::models::sets::Set;

    #[test]
    fn test_set(){
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
        assert_eq!(t_completed_value, t_set.completed_value)
    }
}
