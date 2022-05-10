use diesel::{pg::PgConnection, r2d2::ConnectionManager};

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub struct DbConnection {
    pub db_name: String,
}

impl DbConnection {
    pub fn get_pool(&self) -> DbPool {
        let database_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let database = format!("{}/{}", database_url, &self.db_name);

        let manager = ConnectionManager::<PgConnection>::new(&database);

        r2d2::Pool::new(manager).unwrap()
    }
}
