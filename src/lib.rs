#[macro_use]
pub extern crate diesel;

pub use diesel::prelude::*;
pub use diesel::mysql::MysqlConnection;
pub use diesel::r2d2::ConnectionManager;

pub mod models;

pub use crate::models::*;

pub mod enums;


pub mod schema;

pub fn format_database_url(username: &String, password: &String, address: &String, database_name: &String) -> String {
    format!("mysql://{}:{}@{}/{}", username, password, address, database_name)
}

pub fn establish_connection(database_url: String) -> Result<MysqlConnection, ConnectionError> {
    MysqlConnection::establish(&database_url)
}


#[cfg(test)]
mod tests {
    use dotenv::dotenv;
    use std::env;
    use crate::{MysqlConnection, UserPushConfig, Event, UserEvent};
    use crate::format_database_url;
    use crate::establish_connection;
    use crate::User;
    use crate::diesel::prelude::*;
    use crate::schema::users::dsl as user_dsl;
    use crate::schema::events::dsl as event_dsl;
    use crate::schema::userevents::dsl as user_event_dsl;
    use crate::schema::userpushconfig::dsl as user_push_config_dsl;

    const TEST_DB_ERROR: &str = "\nTo run the tests, you need to have a local mysql database set up. \n\
    You then need to provide an access URL in parts.\n\
    You need: \n\n\
    DATABASE_USER=<username>\n\
    DATABASE_PASSWORD=<password> \n\
    DATABASE_ADDRESS=<ip_of_database>\n\
    Are your credentials correct?";

    fn get_test_conn() -> MysqlConnection {
        dotenv().ok();
        let db_url = format_database_url(
            &env::var("DATABASE_USER").expect(TEST_DB_ERROR),
            &env::var("DATABASE_PASSWORD").expect(TEST_DB_ERROR),
            &env::var("DATABASE_ADDRESS").expect(TEST_DB_ERROR),
            &env::var("DATABASE_NAME").expect(TEST_DB_ERROR),
        );
        let conn = establish_connection(db_url).unwrap();
        conn
    }

    #[test]
    fn test_establish_connection() {
        get_test_conn();
    }

    // The following tests each pull the content of one table, provoking some diesel errors that can
    // only be found at runtime - such as some wrong datatypes, i.e. a date instead of a datetime
    #[test]
    fn test_get_users_content() {
        dotenv().ok();
        let conn = get_test_conn();
        let _us: Vec<User> = user_dsl::users.load::<User>(&conn).unwrap();
    }

    #[test]
    fn test_get_events_content() {
        dotenv().ok();
        let conn = get_test_conn();
        event_dsl::events.load::<Event>(&conn).unwrap();
    }

    #[test]
    fn test_get_user_event_content() {
        dotenv().ok();
        let conn = get_test_conn();
        user_event_dsl::userevents.load::<UserEvent>(&conn).unwrap();
    }

    #[test]
    fn test_get_user_push_config_content() {
        dotenv().ok();
        let conn = get_test_conn();
        user_push_config_dsl::userpushconfig.load::<UserPushConfig>(&conn).unwrap();
    }
}
