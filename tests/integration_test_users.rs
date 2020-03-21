#![allow(non_snake_case)]

extern crate diesel;
extern crate dotenv;
mod tests {
    use DED_backend::establish_connection;
    use DED_backend::models::users::{User,UserList,NewUser};

    use diesel::RunQueryDsl;

    // to run all tests sequentially    cargo test -- --test-threads=1
    #[test]
    fn test_db_user_insert_and_find() {
    // NOTE: these two tests are run sequentially since multi thread test might create a race cond.
        let conn = establish_connection().get().unwrap();

        // delete all entries in the database
        let _xxx = diesel::delete(DED_backend::schema::users::dsl::users)
            .execute(&conn);

        let t_uname = "TestUser";
        let t_fname = "Usable User";
        let t_email = "testuser@testdomain.com";
        let t_salt = "some_like_MSG";

        let t_user = NewUser {
            username: t_uname.to_string(),
            fname: t_fname.to_string(),
            email: t_email.to_string(),
            salt: t_salt.to_string()
        };

        let new_user_id = match t_user.create(&conn) {
            Ok(r_user) => {
                assert_eq!(t_uname, r_user.username);
                assert_eq!(t_fname, r_user.fname);
                assert_eq!(t_email, r_user.email);
                assert_eq!(t_salt, r_user.salt);
                r_user.id
            }
            Err(E) => {//todo: need to fix this
                assert_eq!(1,5);
                print!("got error {}",E);
                // assert_eq!(E, diesel::ConnectionError::CouldntSetupConfiguration);
                0
            }
        };

        match User::get_user(new_user_id, &conn) {
            Ok(r_user) =>{
                assert_eq!(new_user_id, r_user.id);
                assert_eq!(t_uname,r_user.username);
                assert_eq!(t_fname, r_user.fname);
                assert_eq!(t_email, r_user.email);
                assert_eq!(t_salt, r_user.salt);
            }
            Err(E) =>{
                assert_eq!(E, diesel::NotFound);
            }
        }

    }


    #[test]
    fn test_db_user_not_found(){
        let conn = establish_connection().get().unwrap();
        let result = User::get_user(-99, &conn);

        match result{//todo: need to fix this
            Err(E) =>{
                assert_eq!(E, diesel::NotFound);
            }
            Ok(_T) =>{
                print!("Negative user Id in user table");
                assert_eq!(6,-1);
            }
        }
    }


    fn compare_users(u1:User, u2:User){
        assert_eq!(u1.id, u2.id);
        assert_eq!(u1.username, u2.username);
        assert_eq!(u1.fname, u2.fname);
        assert_eq!(u1.email, u2.email);
        assert_eq!(u1.salt, u2.salt);
    }

    #[test]
    fn test_db_user_list(){  //todo:  need to complete this !!!
        let conn = establish_connection().get().unwrap();
        let user_1 = NewUser{
            username: "user 100".to_string(),
            fname: "Weak".to_string(),
            email: "dfsff@2123.com".to_string(),
            salt: "MSG001".to_string()
        };

        let user_2 = NewUser{
            username: "user 200".to_string(),
            fname: "Weak2".to_string(),
            email: "dfsff@21232.com".to_string(),
            salt: "MSG0201".to_string()
        };

        let user_3 = NewUser{
            username: "user 300".to_string(),
            fname: "Weak3".to_string(),
            email: "d3fsff@212532.com".to_string(),
            salt: "MSG30201".to_string()
        };

        user_1.create(&conn);
        user_2.create(&conn);
        user_3.create(&conn);

        let _u_list = UserList::get_users(&conn);

    }
}
