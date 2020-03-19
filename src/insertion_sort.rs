pub fn insertion_sort<T: PartialOrd>(numbers: &mut Vec<T>) {
  for i in 1..numbers.len() {
    for j in (0..i).rev() {
      if numbers[j] > numbers[j + 1] {
        numbers.swap(j, j + 1)
      }
    }
  }
}
