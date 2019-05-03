use super::SortOrder;

pub fn sort<T: Ord>(x: &mut [T], order: &SortOrder) -> Result<(), String> {
  if x.len().is_power_of_two() {
    match *order {
      SortOrder::Ascending => do_sort(x, true),
      SortOrder::Descending => do_sort(x, false),
    };
    Ok(())
  } else {
    Err(format!("The length of x is not expected. it must be the power of 2. current length is {}", x.len()))
  }
}

fn do_sort<T: Ord>(x: &mut [T], up: bool) {
  if x.len() > 1 {
    let mid_point = x.len() / 2;
    do_sort(&mut x[..mid_point], true);
    do_sort(&mut x[mid_point..], false);
    sub_sort(x, up)
  }
}

fn sub_sort<T: Ord>(x: &mut [T], up: bool) {
  if x.len() > 1 {
    compare_and_swap(x, up);
    let mid_point = x.len() / 2;
    sub_sort(&mut x[..mid_point], up);
    sub_sort(&mut x[mid_point..], up);
  }
}

fn compare_and_swap<T: Ord>(x: &mut [T], up: bool) {
  let mid_point = x.len() / 2;
  for i in 0..mid_point {
    if (x[i] > x[mid_point + i]) == up {
      x.swap(i, mid_point + i);
    }
  }
}

#[cfg(test)]
mod tests {
  use super::sort;
  use crate::SortOrder::*;

  #[test]
  fn sort_u_32_ascending() {
    let mut x: Vec<u32> = vec![10, 30, 11, 20, 4, 330, 220, 4440, 10, 1, 444, 4423, 8236, 23, 324, 76];

    assert_eq!(sort(&mut x, &Ascending), Ok(()));

    assert_eq!(x, vec![1, 4, 10, 10, 11, 20, 23, 30, 76, 220, 324, 330, 444, 4423, 4440, 8236]);
  }

  #[test]
  fn sort_u_32_descending() {
    let mut x: Vec<u32> = vec![10, 30, 11, 20, 4, 330, 21, 110];

    assert_eq!(sort(&mut x, &Descending), Ok(()));

    assert_eq!(x, vec![330, 110, 30, 21, 20, 11, 10, 4]);
  }

  #[test]
  fn sort_str_ascending() {
    let mut x = vec!["Rust", "is"];

    assert_eq!(sort(&mut x, &Ascending), Ok(()));

    assert_eq!(x, vec!["Rust", "is"]);
  }

  #[test]
  fn sort_str_descending() {
    let mut x = vec!["Rust", "is"];

    assert_eq!(sort(&mut x, &Descending), Ok(()));

    assert_eq!(x, vec!["is", "Rust"]);
  }

  #[test]
  fn sort_to_fail() {
    let mut x = vec![10, 30, 11];
    assert!(sort(&mut x, &Ascending).is_err())
  }
}
