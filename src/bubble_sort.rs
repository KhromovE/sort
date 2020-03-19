pub fn bubble_sort<T: PartialOrd>(numbers: &mut Vec<T>) {
  let len = numbers.len();
  
  for _ in 0..len {
    for i in 1..len {
      if numbers[i - 1] > numbers[i] {
        numbers.swap(i - 1, i)
      }
    }
  }
}
