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
      if j > 0 { j -= 1; }
    }
  }

  i
}

fn sort<T: PartialOrd + Clone>(numbers: &mut Vec<T>, left: usize, right: usize) -> &mut Vec<T> {
  let index = partition(numbers, left, right);

  if left < index - 1 {
    sort(numbers, left, index - 1);
  }

  if index < right {
    sort(numbers, index, right);
  }

  numbers
}

pub fn quick_sort<T: PartialOrd + Clone>(numbers: &mut Vec<T>) {
  let len = numbers.len();
  
  if len > 1 {
    let left = 0;
    let right = len - 1;
    
    sort(numbers, left, right);
  }
}
