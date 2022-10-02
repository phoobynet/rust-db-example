use rusqlite::{Connection, Error, Result};

#[derive(Debug)]
pub struct Person {
    pub id: Option<u32>,
    pub first_name: String,
    pub last_name: String,
}

pub struct PersonRepository {
    conn: Connection,
}

impl PersonRepository {
    pub async fn connect(path: &str) -> Result<Self, Error> {
        let conn = Connection::open(path)?;

        let s = Self { conn };

        s.create_table().await?;

        Ok(s)
    }

    pub async fn insert(&self, first_name: &str, last_name: &str) -> Result<usize, Error> {
        let id = self
            .conn
            .execute(
                r#"insert into person (first_name, last_name) values ($1, $2)"#,
                &[first_name, last_name],
            )
            .unwrap();

        Ok(id)
    }

    pub async fn find_by_id(&self, id: u32) -> Result<Person, Error> {
        let mut stmt = self
            .conn
            .prepare(r#"select id, first_name, last_name from person where id = $1"#)?;

        let person = stmt
            .query_row(&[&id], |row| {
                Ok(Person {
                    id: row.get(0)?,
                    first_name: row.get(1)?,
                    last_name: row.get(2)?,
                })
            })
            .unwrap();

        Ok(person)
    }

    async fn create_table(&self) -> Result<(), Error> {
        let _ = self.conn.execute(
            r#"
            CREATE TABLE IF NOT EXISTS person (
                id integer primary key,
                first_name text,
                last_name text
            );
            "#,
            [],
        )?;

        Ok(())
    }
}
