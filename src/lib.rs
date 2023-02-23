use chrono::{Local, NaiveDate};
use core::fmt;

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
