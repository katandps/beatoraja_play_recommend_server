use mysql::MySQLClient;

pub fn main() {
    let repository = MySQLClient::new();
    repository.run();
}
