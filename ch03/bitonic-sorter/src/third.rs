use std::cmp::Ordering;

pub fn sort_by<T, F>(x: &mut [T], comparator: &F) -> Result<(), String>
  where F: Fn(&T, &T) -> Ordering
{
  if is_power_of_two(x.len()) {
    do_sort(x, true, comparatore);
    Ok(())
  } else {
    Err(format!("The length of x is not a powe of two. {}", x.len()))
  }
}

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

  #[test]
  fn sort_students_by_name_ascending() {
    let s1 = Student::new("s1first", "s1last", 16);
    let s2 = Student::new("s2first", "s2last", 14);
    let s3 = Student::new("s3first", "s3last", 15);
    let s4 = Student::new("s4first", "s4last", 23);

    let mut x = vec![&s2, &s1, &s4, &s3];

    let expected = vec![&s1, &s2, &s3, &s4];

    assert_eq!(
      sort_by(&mut x, &|a, b|
        a.last_name.cmp(&b.last_name).then_with(|| a.frist_name.cmp(&b.first_name))
      ),
      Ok(())
    );
    assert_eq!(x, expected);
  }
}