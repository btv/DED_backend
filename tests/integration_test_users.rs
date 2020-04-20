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

        let t_uname = "TestUser";
        let t_name = "Usable_User";
        let t_email = "testuser@testdomain.com";
        let t_passwd = "some_like_MSG";

        let t_user = NewUser {
            username: t_uname.to_string(),
            name: t_name.to_string(),
            email: t_email.to_string(),
            passwd: t_passwd.to_string()
        };

        let new_user_id = match t_user.create(&conn) {
            Ok(r_user) => {
                assert_eq!(t_uname, r_user.username);
                assert_eq!(t_name, r_user.name);
                assert_eq!(t_email, r_user.email);
                assert_eq!(t_passwd, r_user.passwd);
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
                assert_eq!(r_user, t_user)
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

    #[test]
    fn test_db_user_list(){
        let conn = establish_connection().get().unwrap();

        for x in 1..101 {
            let newUser = NewUser{
                username: format!("username{}", x).to_string(),
                name: format!("name_number{}",x).to_string(),
                email: format!("user{}@colorado.edu",x).to_string(),
                passwd: format!("slatyas#{}",x).to_string()
            };
            newUser.create(&conn);
        }


         let u_list = UserList::get_users(&conn);

        assert!(u_list.0.len() > 90);
    }
}
