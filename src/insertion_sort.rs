pub fn insertion_sort<T: PartialOrd + Clone>(numbers: &Vec<T>) -> Vec<T> {
  let mut sorted: Vec<T> = numbers.clone();
  
  for i in 1..sorted.len() {
    for j in (0..i).rev() {
      if sorted[j] > sorted[j + 1] {
        sorted.swap(j, j + 1)
      }
    }
  }

  sorted
}


#[cfg(test)]
mod tests {
  use super::insertion_sort;
    #[test]    
    fn test_insertion_sort() {
      assert_eq!(insertion_sort(&vec![2, 1, 9, 20, 4]), vec![1, 2, 4, 9, 20]);
      assert_eq!(insertion_sort(&vec![3, 2, 2, 2, 1, 1, 9, 29]), vec![1, 1, 2, 2, 2, 3, 9, 29]);
    }
}