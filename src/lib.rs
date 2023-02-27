
use sqlx::types::chrono::Local;
use core::fmt;
use sqlx::{database::HasArguments, Database, Executor, IntoArguments};
use sqlx::{Encode, Type, Row, FromRow, Decode, ColumnIndex};

#[derive(Debug)]
pub struct JobApplication {
    pub id: i64,
    pub name: String,
    pub date: String,
    pub resume_sent: bool,
    pub coverletter_sent: bool,
    pub response_date: Option<String>,
    pub interview_date: Option<String>,
}

impl fmt::Display for JobApplication {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ID: {}\nName: {}\nApplication Sent: {}\nResume Sent?: {}\nCover Letter Sent?: {}\nResponse Received: {}\nInterview Date: {}\n",
            self.id,
            self.name,
            self.date,
            if self.resume_sent { "yes" } else { "no" },
            if self.coverletter_sent { "yes" } else { "no" },
            if let Some(date) = &self.response_date { date.to_string() } else { String::from("no") },
            if let Some(date) = &self.interview_date { date.to_string() } else { String::from("no") }
        )
    }
}

impl <'r, R> FromRow<'r, R> for JobApplication
where
    R: Row,
    for<'c> &'c str: ColumnIndex<R>,
    for<'c> String: Decode<'c, R::Database> + Type<R::Database>,
    for<'c> bool: Decode<'c, R::Database> + Type<R::Database>,
    for<'c> Option<String>: Decode<'c, R::Database> + Type<R::Database>,
    for<'c> i64: Decode<'c, R::Database> + Type<R::Database>,
{
    fn from_row(row: &'r R) -> Result<Self, sqlx::Error> {
        let mut app = JobApplication::new(
            row.try_get("name")?,
            row.try_get("date")?,
            row.try_get("resume_sent")?,
            row.try_get("coverletter_sent")?,
            row.try_get("response_date")?,
            row.try_get("interview_date")?
        );
        app.id = row.try_get("id")?;
        Ok(app)
    }
}

impl JobApplication {
    pub fn new(
        name: String,
        date_applied: Option<String>,
        resume_sent: bool,
        coverletter_sent: bool,
        response_date: Option<String>,
        interview_date: Option<String>,
    ) -> Self {
        Self {
            id: 0,
            name,
            date: if let Some(date) = date_applied {
                date
            } else {
                Local::now().date_naive().to_string()
            },
            resume_sent,
            coverletter_sent,
            response_date,
            interview_date,
        }
    }

    pub async fn create<'a, DB, Pool>(&'a mut self, pool: Pool) -> Result<(), sqlx::Error>
    where
        DB: Database,
        <DB as HasArguments<'a>>::Arguments: IntoArguments<'a, DB>,
        Pool: 'a + Executor<'a, Database = DB>,
        for<'c> String: Encode<'c, DB> + Type<DB>,
        for<'c> bool: Encode<'c, DB> + Type<DB>,
        for<'c> Option<String>: Encode<'c, DB> + Type<DB>,
        for<'c> i64: Decode<'c, DB> + Type<DB>,
        for<'c> usize: ColumnIndex<DB::Row>,
        for<'c> &'c str: ColumnIndex<DB::Row>,
    {
        let result = sqlx::query(
        r#"
        INSERT INTO application (name, date, resume_sent, coverletter_sent, response_date, interview_date)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6)
        RETURNING id
        "#)
        .bind(&self.name)
        .bind(&self.date)
        .bind(self.resume_sent)
        .bind(self.coverletter_sent) 
        .bind(&self.response_date)
        .bind(&self.interview_date)
        .fetch_all(pool)
        .await?;
        self.id = result[0].try_get("id")?;
        Ok(())
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
