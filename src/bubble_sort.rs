pub fn bubble_sort<T: PartialOrd + Clone>(numbers: &Vec<T>) -> Vec<T> {
  let mut sorted: Vec<T> = numbers.clone();
  let len = sorted.len();
  
  for _ in 0..len {
    for i in 1..len {
      if sorted[i - 1] > sorted[i] {
        sorted.swap(i - 1, i)
      }
    }
  }

  sorted
}


#[cfg(test)]
mod tests {
    use super::bubble_sort;
     #[test]
     fn test_bubble_sort() {
      assert_eq!(bubble_sort(&vec![2, 1, 9, 20, 4]), vec![1, 2, 4, 9, 20]);
      assert_eq!(bubble_sort(&vec![3, 2, 2, 2, 1, 1, 9, 29]), vec![1, 1, 2, 2, 2, 3, 9, 29]);
    }

}