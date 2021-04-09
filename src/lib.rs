#[macro_use]
pub extern crate diesel;

pub mod models;
pub use crate::models::*;
pub mod enums;

pub use diesel::prelude::*;
use diesel::mysql::MysqlConnection;

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
    use crate::MysqlConnection;
    use crate::format_database_url;
    use crate::establish_connection;
    use crate::User;
    use crate::diesel::prelude::*;
    use crate::schema::users::dsl;

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

    #[test]
    fn test_get_db_content() {
        dotenv().ok();
        let conn = get_test_conn();
        let _us: Vec<User> = dsl::users.filter(dsl::isActive.eq(true)).load::<User>(&conn).unwrap();
    }
}
