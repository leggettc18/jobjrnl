use chrono::{DateTime, Local};
use core::fmt;

#[derive(Debug)]
pub struct JobApplication {
    name: String,
    date_applied: DateTime<Local>,
    resume_sent: bool,
    cover_letter_sent: bool,
    response_received: Option<DateTime<Local>>,
    interview_date: Option<DateTime<Local>>,
}

impl fmt::Display for JobApplication {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Name: {}\nApplication Sent: {}\nResume Sent?: {}\nCover Letter Sent?: {}\nResponse Received: {}\nInterview Date: {}\n",
            self.name,
            self.date_applied,
            if self.resume_sent { "yes" } else { "no" },
            if self.cover_letter_sent { "yes" } else { "no" },
            if let Some(date) = self.response_received { date.to_string() } else { String::from("no") },
            if let Some(date) = self.interview_date { date.to_string() } else { String::from("no") }
        )
    }
}

impl JobApplication {
    pub fn new(
        name: String,
        date_applied: DateTime<Local>,
        resume_sent: bool,
        cover_letter_sent: bool,
        response_received: Option<DateTime<Local>>,
        interview_date: Option<DateTime<Local>>,
    ) -> Self {
        Self {
            name,
            date_applied,
            resume_sent,
            cover_letter_sent,
            response_received,
            interview_date,
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
