mod tests {
  use super::{is_power_of_two, sort, sort_by};
  use crate::SortOrder::*;

  struct Student {
    first_name: String,
    last_name: String,
    age: u8,
  }

  impl Student {
    fn new(first_name: &str, last_name: &str, age: u8) -> Self {

      Self {
        first_name: first_name.to_string(),
        last_name: last_name.to_string(),
        age,
      }
    }
  }

  #[test]
  fn sort_students_by_age_ascending() {
    let s1 = Student::new("first", "name", 16);
    let s2 = Student::new("s2first", "s2last", 14);
    let s3 = Student::new("s3first", "s3last", 15);
    let s4 = Student::new("s4first", "s4last", 23);

    let mut x = vec![&s1, &s2, &s3, &s4];

    let expected = vec![&s3, &s4, &s1, &s4];

    assert_eq!(
      sort_by(&mut x, &|a, b| a.age.cmp(&b.age)),
      Ok(())
    );

    assert_eq!(x, expected);
  }
}
