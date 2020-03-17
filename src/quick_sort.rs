fn partition<T: PartialOrd + Clone>(numbers: &mut Vec<T>, left: usize, right: usize) -> usize {
  let pivot_index = (left + right) / 2;
  let pivot = numbers[pivot_index].clone();
  let mut i = left;
  let mut j = right;

  while i <= j {
    while numbers[i] < pivot {
      i += 1;
    }

    while numbers[j] > pivot {
      j -= 1;
    }

    if i <= j {
      numbers.swap(i, j);
      i += 1;
      j -= 1;
    }
  }

  i
}

fn _qucik_sort<T: PartialOrd + Clone>(sorted: &mut Vec<T>, left: usize, right: usize) -> &mut Vec<T> {
  if sorted.len() > 1 {
    let index = partition(sorted, left, right);

    if left < index - 1 {
      _qucik_sort(sorted, left, index - 1);
    }

    if index < right {
      _qucik_sort(sorted, index, right);
    }
   
  }

  sorted
}

pub fn quick_sort<T: PartialOrd + Clone>(numbers: &Vec<T>) -> Vec<T> {
  let mut sorted: Vec<T> = numbers.clone();
  let left = 0;
  let len = sorted.len();
  let right = len - 1;
  
  _qucik_sort(&mut sorted, left, right);

  sorted
}


#[cfg(test)]
mod tests {
  use super::quick_sort;
    #[test]    
    fn test_quick_sort() {
      assert_eq!(quick_sort(&vec![2, 1, 9, 20, 4]), vec![1, 2, 4, 9, 20]);
      assert_eq!(quick_sort(&vec![3, 2, 2, 2, 1, 1, 9, 29]), vec![1, 1, 2, 2, 2, 3, 9, 29]);
    }
}