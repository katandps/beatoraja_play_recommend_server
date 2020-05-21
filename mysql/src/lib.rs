mod schema;

use beatoraja_play_recommend::*;
use diesel::prelude::*;
use std::env;

#[macro_use]
extern crate diesel;
extern crate dotenv;

pub struct MySQLClient {}

impl MySQLClient {
    pub fn new() -> MySQLClient {
        MySQLClient {}
    }

    fn establish_connection() -> MysqlConnection {
        dotenv::dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        MysqlConnection::establish(&database_url)
            .unwrap_or_else(|_| panic!("Error connection to {}", &database_url))
    }
}

impl ScoreRepository for MySQLClient {
    fn score(&self) -> Scores {
        unimplemented!()
    }
}
impl SongRepository for MySQLClient {
    fn song_data(&self) -> Songs {
        unimplemented!()
    }
}
impl ScoreLogRepository for MySQLClient {
    fn score_log(&self) -> ScoreLog {
        use schema::score_logs::dsl::*;
        let connection = Self::establish_connection();
        let record = score_logs
            .load::<schema::score_log::ScoreLog>(&connection)
            .expect("Error loading schema");

        let mut builder = ScoreLogBuilder::builder();
        if record.len() != 0 {
            let row = record.get(0).unwrap();
            let logs: Vec<SnapShots> = serde_json::from_str(&row.data).unwrap();
            for log in logs {
                builder.push_snapshots(log);
            }
        }
        builder.build()
    }
}
