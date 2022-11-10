#[derive(Debug)]
pub struct JobApplication {
    name: String,
    date_applied: String,
    resume_sent: bool,
    cover_letter_sent: bool,
    response_received: Option<String>,
    interview_date: Option<String>,
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
