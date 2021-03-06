use serde::{Serialize, Deserialize};
use std::time::SystemTime;

use crate::schema::sets;
use diesel::query_dsl::filter_dsl::FindDsl;
use diesel::PgConnection;

#[derive(
    PartialEq, Debug, Clone, Queryable, Identifiable, Insertable,
    AsChangeset, QueryableByName, Serialize, Deserialize
)]
#[table_name = "sets"]
pub struct Set {
    pub id: i32,
    pub exercise_id: i32,
    pub style: String,
    pub unit: String,
    pub goal_reps: i16,
    pub goal_value: i16,
    pub description: String,
    pub created_or_completed: SystemTime,
    pub completed_reps: i16,
    pub completed_value: i16,
    pub origin_id: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable, AsChangeset)]
#[table_name = "sets"]
pub struct NewSet {
    pub exercise_id:i32,
    pub style: String,
    pub unit: String,
    pub goal_reps: i16,
    pub goal_value: i16,
    pub description: String,
    pub origin_id: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetList( pub Vec<Set> );

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompleteSet {
    pub completed_reps: i16,
    pub completed_value: i16,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable, AsChangeset)]
#[table_name = "sets"]
struct CompleteSetFull {
    pub completed_reps: i16,
    pub completed_value: i16,
    pub created_or_completed: SystemTime,
}

impl From<&CompleteSet> for CompleteSetFull {
    fn from(cs: &CompleteSet) -> Self {
        CompleteSetFull {
            completed_reps: cs.completed_reps,
            completed_value: cs.completed_value.clone(),
            created_or_completed: SystemTime::now()
        }
    }

}
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

    pub fn delete(in_id: i32, connection: &PgConnection) -> Result<usize, diesel::result::Error> {
        use diesel::RunQueryDsl;

        diesel::delete(sets::table.find(in_id)).execute(connection)
    }

    pub fn update(in_id: i32, new_set: &NewSet, connection: &PgConnection) -> Result<usize, diesel::result::Error> {
        use diesel::RunQueryDsl;

        diesel::update(sets::table.find(in_id))
            .set(new_set)
            .execute(connection)
    }

    pub fn complete(in_id: i32, comp_set: &CompleteSet, connection: &PgConnection) -> Result<usize, diesel::result::Error> {
        use diesel::RunQueryDsl;

        diesel::update(sets::table.find(in_id))
            .set(CompleteSetFull::from(comp_set))
            .execute(connection)
    }
}

impl PartialEq<NewSet> for Set {
    fn eq(&self, other:& NewSet) -> bool {
        self.exercise_id == other.exercise_id &&
        self.style == other.style &&
        self.unit == other.unit &&
        self.goal_reps == other.goal_reps &&
        self.goal_value == other.goal_value &&
        self.description == other.description &&
        self.origin_id == other.origin_id
    }
}

impl SetList {
    pub fn get_sets_by_exercise_id(ex_id: i32, conn: &PgConnection) -> Result<Vec<Set>, diesel::result::Error> {
        use diesel::prelude::*;
        use crate::schema::sets::dsl::exercise_id;

        sets::table.filter(exercise_id.eq(ex_id))
            .get_results::<Set>(conn)
    }

    pub fn get_sets_by_origin_id(ex_id: i32, conn: &PgConnection) -> Result<Vec<Set>, diesel::result::Error> {
        use diesel::prelude::*;
        use crate::schema::sets::dsl::exercise_id;

        sets::table.filter(exercise_id.eq(ex_id))
            .get_results::<Set>(conn)
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
        let t_goal_value = 10;
        let t_description = "Twist and shout";
        let t_created_or_completed = SystemTime::now();
        let t_completed_reps = 10;
        let t_completed_value = 10;
        let t_origin_id = 10;

        let t_set = Set{
            id: t_id,
            exercise_id: t_exercise_id,
            style: t_style.to_string(),
            unit: t_unit.to_string(),
            goal_reps: t_goal_reps,
            goal_value: t_goal_value,
            description:t_description.to_string(),
            created_or_completed: t_created_or_completed,
            completed_reps: t_completed_reps,
            completed_value: t_completed_value,
            origin_id: t_origin_id
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
        assert_eq!(t_origin_id, t_set.origin_id)
    }
}
