use chrono::{Local, NaiveDate};
use core::fmt;
use sqlx::{database::HasArguments, Database, Executor, IntoArguments};
use sqlx::{Encode, Type};

#[derive(Debug)]
pub struct JobApplication {
    pub name: String,
    pub date_applied: NaiveDate,
    pub resume_sent: bool,
    pub coverletter_sent: bool,
    pub response_received: Option<NaiveDate>,
    pub interview_date: Option<NaiveDate>,
}

impl fmt::Display for JobApplication {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Name: {}\nApplication Sent: {}\nResume Sent?: {}\nCover Letter Sent?: {}\nResponse Received: {}\nInterview Date: {}\n",
            self.name,
            self.date_applied,
            if self.resume_sent { "yes" } else { "no" },
            if self.coverletter_sent { "yes" } else { "no" },
            if let Some(date) = self.response_received { date.to_string() } else { String::from("no") },
            if let Some(date) = self.interview_date { date.to_string() } else { String::from("no") }
        )
    }
}

impl JobApplication {
    pub fn new(
        name: String,
        date_applied: Option<String>,
        resume_sent: bool,
        coverletter_sent: bool,
        response_received: Option<String>,
        interview_date: Option<String>,
    ) -> Self {
        Self {
            name,
            date_applied: if let Some(date) = date_applied {
                NaiveDate::parse_from_str(date.as_ref(), "%Y-%m-%d").unwrap()
            } else {
                Local::now().date_naive()
            },
            resume_sent,
            coverletter_sent,
            response_received: response_received
                .map(|date| NaiveDate::parse_from_str(date.as_ref(), "%Y-%m-%d").unwrap()),
            interview_date: interview_date
                .map(|date| NaiveDate::parse_from_str(date.as_ref(), "%Y-%m-%d").unwrap()),
        }
    }

    pub async fn save<'a, DB, E>(&'a self, e: E) -> Result<(), sqlx::Error>
    where
        DB: Database,
        <DB as HasArguments<'a>>::Arguments: IntoArguments<'a, DB>,
        for<'c> E: 'a + Executor<'c, Database = DB>,
        for<'c> String: Encode<'c, DB> + Type<DB>,
        for<'c> chrono::NaiveDate: Encode<'c, DB> + Type<DB>,
        for<'c> bool: Encode<'c, DB> + Type<DB>,
        for<'c> Option<chrono::NaiveDate>: Encode<'c, DB> + Type<DB>,
    {
        sqlx::query(
        r#"
        INSERT INTO application (name, date, resume_sent, coverletter_sent, response_date, interview_date)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6);
        "#)
        .bind(&self.name)
        .bind(self.date_applied)
        .bind(self.resume_sent)
        .bind(self.coverletter_sent) 
        .bind(self.response_received)
        .bind(self.interview_date)
        .execute(e)
        .await?;

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
