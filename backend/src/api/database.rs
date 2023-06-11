use rocket_db_pools::mongodb::Client;
use rocket_db_pools::Database;

#[derive(Database)]
#[database("notes")]
pub struct NotesDB(pub Client);
