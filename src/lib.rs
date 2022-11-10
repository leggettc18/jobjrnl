use core::fmt;

#[derive(Debug)]
pub struct JobApplication {
    name: String,
    date_applied: String,
    resume_sent: bool,
    cover_letter_sent: bool,
    response_received: Option<String>,
    interview_date: Option<String>,
}

impl fmt::Display for JobApplication {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Name: {}\nApplication Sent: {}\nResume Sent?: {}\nCover Letter Sent?: {}\nResponse Received: {}\nInterview Date: {}\n",
            self.name, self.date_applied, if self.resume_sent { "yes" } else { "no" }, if self.cover_letter_sent { "yes" } else { "no" },
            self.response_received.as_ref().unwrap_or(&String::from("n/a")), self.interview_date.as_ref().unwrap_or(&String::from("n/a"))
        )
    }
}

impl JobApplication {
    pub fn new(
        name: String,
        date_applied: String,
        resume_sent: bool,
        cover_letter_sent: bool,
        response_received: Option<String>,
        interview_date: Option<String>,
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
