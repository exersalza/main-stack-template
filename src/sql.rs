pub(crate) struct Sql {
    conn: sqlx::Pool<sqlx::MySql>,
}

impl Sql {
    pub fn new() -> Self {
        todo!()
    }
}

impl Default for Sql {
    fn default() -> Self {
        Self::new()
    }
}
