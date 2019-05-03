use std::cmp::Ordering;
use super::SortOrder;

pub fn sort<T: Ord>(x: &mut [T], order: &SortOrder) -> Result<(), String> {
  if x.len().is_power_of_two() {
    match *order {
      SortOrder::Ascending => sort_by(x, &|a, b| a.cmp(b)),
      SortOrder::Descending => sort_by(x, &|a, b| b.cmp(a)),
    };
    Ok(())
  } else {
    Err(format!("The length of x is not expected. it must be the power of 2. current length is {}", x.len()))
  }
}

pub fn sort_by<T, F>(x: &mut [T], comparator: &F) -> Result<(), String>
  where F: Fn(&T, &T) -> Ordering
{
  if x.len().is_power_of_two() {
    do_sort(x, true, comparator);
    Ok(())
  } else {
    Err(format!("The length of x is not a powe of two. {}", x.len()))
  }
}

fn do_sort<T: Ord, F>(x: &mut [T], forward: bool, comparator: &F)
  where F: Fn(&T, &T) -> Ordering
{
  if x.len() > 1 {
    let mid_point = x.len() / 2;
    do_sort(&mut x[..mid_point], true, comparator);
    do_sort(&mut x[mid_point..], false, comparator);
    sub_sort(x, forward, comparator)
  }
}

fn sub_sort<T: Ord, F>(x: &mut [T], forward: bool, comparator: &F)
  where F: Fn(&T, &T) -> Ordering
{
  if x.len() > 1 {
    compare_and_swap(x, forward, comparator);
    let mid_point = x.len() / 2;
    sub_sort(&mut x[..mid_point], forward, comparator);
    sub_sort(&mut x[mid_point..], forward, comparator);
  }
}

fn compare_and_swap<T: Ord, F>(x: &mut [T], forward: bool, compare_and_swap: &F)
  where F: Fn(&T, &T) -> Ordering
{
  let swap_condition = if forward {
    Ordering::Greater
  } else {
    Ordering::Less
  };
  let mid_point = x.len() / 2;
  for i in 0..mid_point {
    if (x[i] > x[mid_point + i]) == swap_condition {
      x.swap(i, mid_point + i);
    }
  }
}

mod tests {
  use super::{sort, sort_by};
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
        a.last_name.cmp(&b.last_name).then_with(|| a.first_name.cmp(&b.first_name))
      ),
      Ok(())
    );
    assert_eq!(x, expected);
  }
}
