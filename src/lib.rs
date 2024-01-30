use async_trait::async_trait;
use core::fmt;
use sqlx::types::chrono::Local;
use sqlx::{ColumnIndex, Decode, FromRow, Row, Type};

#[derive(Debug)]
pub struct JobApplication {
    pub id: i64,
    pub name: String,
    pub date: String,
    pub description: String,
    pub resume_sent: bool,
    pub coverletter_sent: bool,
    pub response_date: Option<String>,
    pub interview_date: Option<String>,
}

impl fmt::Display for JobApplication {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ID: {}\nName: {}\nApplication Sent: {}\nResume Sent?: {}\nCover Letter Sent?: {}\nResponse Received: {}\nInterview Date: {}\nDescription: {}",
            self.id,
            self.name,
            self.date,
            if self.resume_sent { "yes" } else { "no" },
            if self.coverletter_sent { "yes" } else { "no" },
            if let Some(date) = &self.response_date { date.to_string() } else { String::from("no") },
            if let Some(date) = &self.interview_date { date.to_string() } else { String::from("no") },
            self.description
        )
    }
}

#[derive(Debug)]
pub struct JobApplicationList(pub Vec<JobApplication>);

impl fmt::Display for JobApplicationList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "ID   | Application Sent | Resume Sent? | Cover Letter Sent? | Response Received | Interview Date",
        )?;
        writeln!(
            f,
            "-----|------------------|--------------|--------------------|-------------------|---------------",
        )?;
        let apps = &self.0;
        for (_, app) in apps.iter().enumerate() {
            writeln!(
                f,
                "{:<4} | {:<16} | {:<12} | {:<18} | {:<17} | {:<14}",
                app.id,
                app.date,
                app.resume_sent,
                app.coverletter_sent,
                if let Some(date) = &app.response_date {
                    date.to_string()
                } else {
                    String::from("no")
                },
                if let Some(date) = &app.interview_date {
                    date.to_string()
                } else {
                    String::from("no")
                }
            )?;
        }
        writeln!(f, "Count: {}", &self.0.len())
    }
}

impl<'r, R> FromRow<'r, R> for JobApplication
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
            row.try_get("description")?,
            row.try_get("resume_sent")?,
            row.try_get("coverletter_sent")?,
            row.try_get("response_date")?,
            row.try_get("interview_date")?,
        );
        app.id = row.try_get("id")?;
        Ok(app)
    }
}

impl JobApplication {
    pub fn new(
        name: String,
        description: Option<String>,
        date_applied: Option<String>,
        resume_sent: bool,
        coverletter_sent: bool,
        response_date: Option<String>,
        interview_date: Option<String>,
    ) -> Self {
        Self {
            id: 0,
            name,
            description: if let Some(description) = description {
                description
            } else {
                "".to_string()
            },
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
}

#[async_trait]
pub trait CRUDable<DB: sqlx::Database> {
    async fn create(&mut self, db: &sqlx::Pool<DB>) -> Result<(), sqlx::Error>;
    async fn list(db: &sqlx::Pool<DB>) -> Result<Vec<JobApplication>, sqlx::Error>;
    async fn get(db: &sqlx::Pool<DB>, id: i64) -> Result<JobApplication, sqlx::Error>;
    async fn update(&self, db: &sqlx::Pool<DB>) -> Result<(), sqlx::Error>;
    async fn delete(&self, db: &sqlx::Pool<DB>) -> Result<(), sqlx::Error>;
}

#[async_trait]
impl CRUDable<sqlx::Sqlite> for JobApplication {
    async fn create(&mut self, db: &sqlx::Pool<sqlx::Sqlite>) -> Result<(), sqlx::Error> {
        let result = sqlx::query!(
            r#"
            INSERT INTO application (name, date, description, resume_sent, coverletter_sent, response_date, interview_date)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
            "#,
            self.name, self.date, self.description, self.resume_sent, self.coverletter_sent, self.response_date, self.interview_date
        )
        .execute(db)
        .await?;
        self.id = result.last_insert_rowid();
        Ok(())
    }

    async fn list(db: &sqlx::Pool<sqlx::Sqlite>) -> Result<Vec<JobApplication>, sqlx::Error> {
        let recs = sqlx::query_as!(
            JobApplication,
            r#"
            SELECT id, name, date, description, resume_sent, coverletter_sent, response_date, interview_date
            FROM application
            ORDER BY id
            "#
        )
        .fetch_all(db)
        .await?;
        Ok(recs)
    }

    async fn get(db: &sqlx::Pool<sqlx::Sqlite>, id: i64) -> Result<JobApplication, sqlx::Error> {
        let result = sqlx::query_as!(
            JobApplication,
            r#"
            SELECT id, name, date, description, resume_sent, coverletter_sent, response_date, interview_date
            FROM application
            WHERE id = ?1
            ORDER BY id
            "#,
            id
        )
        .fetch_one(db)
        .await?;
        Ok(result)
    }

    async fn update(&self, _db: &sqlx::Pool<sqlx::Sqlite>) -> Result<(), sqlx::Error> {
        todo!();
    }

    async fn delete(&self, _db: &sqlx::Pool<sqlx::Sqlite>) -> Result<(), sqlx::Error> {
        todo!();
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
