#[derive(Debug)]
pub struct JobApplication {
    pub name: String,
    pub date_applied: String,
    pub resume_sent: bool,
}

impl JobApplication {
    pub fn new(name: String, date_applied: String, resume_sent: bool) -> Self { Self { name, date_applied, resume_sent } }
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
