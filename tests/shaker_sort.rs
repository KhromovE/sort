extern crate sort;

#[cfg(test)]
mod shaker_sort {
  use sort;

  #[test]
  fn empty_vec_test() {
    let mut unsorted: Vec<i16> = vec![];
    let result = vec![];
    sort::shaker_sort(&mut unsorted);

    assert_eq!(unsorted, result);
  }

  #[test]
  fn one_value_test() {
    let mut unsorted = vec![1];
    let result = vec![1];
    sort::shaker_sort(&mut unsorted);

    assert_eq!(unsorted, result);
  }

  #[test]
  fn two_values_test() {
    let mut unsorted = vec![2, 1];
    let result = vec![1, 2];
    sort::shaker_sort(&mut unsorted);

    assert_eq!(unsorted, result);
  }

  #[test]
  fn numbers_test() {
    let mut unsorted = vec![6, 2, 2, -11, 2, 200, 1];
    let result = vec![-11, 1, 2, 2, 2, 6, 200];
    sort::shaker_sort(&mut unsorted);

    assert_eq!(unsorted, result);
  }
}