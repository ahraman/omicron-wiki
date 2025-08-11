use crate::{Error, db::Db};

pub struct NewUser<'a> {
    pub name: &'a str,
    pub email: &'a str,
}

impl<'a> NewUser<'a> {
    pub fn new(name: &'a str, email: &'a str) -> Self {
        Self { name, email }
    }
}

impl NewUser<'_> {
    pub async fn insert(self, db: &Db) -> Result<(), Error> {
        let _ = sqlx::query(
            "INSERT INTO users (name, email)
                    VALUES ($1, $2)
                    ON CONFLICT ",
        )
        .bind(self.name)
        .bind(self.email)
        .execute(&db.conn)
        .await?;

        Ok(())
    }
}
