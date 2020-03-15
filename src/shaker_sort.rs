pub fn shaker_sort<T: PartialOrd + Clone>(numbers: &Vec<T>) -> Vec<T> {
  let mut sorted: Vec<T> = numbers.clone();
  let len = sorted.len();
  let mut left = 0;
  let mut right = len - 1;

  loop {
    for i in left..right {
      if sorted[i] > sorted[i + 1] {
        sorted.swap(i, i + 1);
      }
    }

    right -= 1;

    for i in (left..right).rev() {
      if left < i && sorted[i] < sorted[i - 1]  {
        sorted.swap(i, i - 1);
      }
    }

    left += 1;

    if left >= right {
      break;
    }
  }

  sorted
}


#[cfg(test)]
mod tests {
    use super::shaker_sort;
     #[test]
    fn test_shaker_sort() {
        assert_eq!(shaker_sort(&vec![2, 1, 9, 20, 4]), vec![1, 2, 4, 9, 20]);
        assert_eq!(shaker_sort(&vec![3, 2, 2, 2, 1, 1, 9, 29]), vec![1, 1, 2, 2, 2, 3, 9, 29]);
    }

}