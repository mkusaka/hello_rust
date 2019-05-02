pub fn sort(x: &mut [u32], up: bool) {
  if x.len() > 1 {
    let mid_point = x.len() / 2;
    sort(&mut x[..mid_point], true);
    sort(&mut x[mid_point..], false);
    sub_sort(x, up)
  }
}

fn sub_sort(x: &mut [u32], up: bool) {
  if x.len() > 1 {
    compare_and_swap(x, up);
    let mid_point = x.len() / 2;
    sub_sort(&mut x[..mid_point], up);
    sub_sort(&mut x[mid_point..], up);
  }
}

fn compare_and_swap(x: &mut [u32], up: bool) {
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

  #[test]
  fn sort_u_32_ascending() {
    let mut x = vec![10, 30, 11, 20, 4, 330, 220, 4440, 10, 1, 444, 4423, 8236, 23, 324, 76];

    sort(&mut x, true);

    assert_eq!(x, vec![1, 4, 10, 10, 11, 20, 23, 30, 76, 220, 324, 330, 444, 4423, 4440, 8236]);
  }

  #[test]
  fn sort_u_32_descending() {
    let mut x = vec![10, 30, 11, 20, 4, 330, 21, 110];

    sort(&mut x, false);

    assert_eq!(x, vec![330, 110, 30, 21, 20, 11, 10, 4]);
  }

  #[test]
  fn sort_u32_descendingorg() {
    let mut x = vec![10, 30, 11, 20, 4, 330, 21, 110];
    sort(&mut x, false);
    // xの要素が降順にソートされていることを確認する
    assert_eq!(x, vec![330, 110, 30, 21, 20, 11, 10, 4]);
  }
}
