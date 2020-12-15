mod models;
mod schema;

use diesel::prelude::*;
use models::*;

#[macro_use]
extern crate diesel;
extern crate anyhow;

pub struct MySQLClient {
    connection: MysqlConnection,
}

impl MySQLClient {
    pub fn new() -> Self {
        Self {
            connection: Self::establish_connection(config().mysql_url()),
        }
    }

    fn establish_connection(url: String) -> MysqlConnection {
        MysqlConnection::establish(&url).expect(&format!("Error connecting to {}", url))
    }

    pub fn run(&self) {
        let new_users = vec![
            NewUser {
                name: String::from("new_user4"),
            },
            NewUser {
                name: String::from("new_user5"),
            },
        ];
        diesel::insert_into(schema::users::dsl::users)
            .values(new_users)
            .execute(&self.connection)
            .expect("Error saving new user");
    }
}

fn config() -> beatoraja_play_recommend::Config {
    if cfg!(test) {
        beatoraja_play_recommend::Config::Dummy
    } else {
        beatoraja_play_recommend::config()
    }
}
