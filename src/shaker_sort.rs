pub fn shaker_sort<T: PartialOrd>(numbers: &mut Vec<T>) {
  let mut left = 0;
  let len = numbers.len();
  let mut right = len;

  if len > 0 {
    loop {
      for i in left..right - 1 {
        if numbers[i] > numbers[i + 1] {
          numbers.swap(i, i + 1);
        }
      }

      right -= 1;

      for i in (left..right).rev() {
        if left < i && numbers[i] < numbers[i - 1]  {
          numbers.swap(i, i - 1);
        }
      }

      left += 1;

      if left >= right {
        break;
      }
    }
  }
}
