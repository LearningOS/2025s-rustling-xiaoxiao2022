/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/


// bubble sort 
// fn sort<T>(array: &mut [T])
// where 
//     T: PartialOrd,  // PartialOrd trait is used to compare two values of the same type
// {
// 	//TODO
//     let len = array.len();
//     for i in 0..len {
//         for j in 0..len - i -1 {
//             if array[j] > array[j + 1] {
//                 array.swap(j, j + 1);
//             }

//         }
//     }
// }

// quick sort
fn sort<T: Ord>(array: &mut [T]) {
    if array.len() <= 1 {
        return;
    }
    let pivot_index = partition(array);
    sort(&mut array[0..pivot_index]);
    sort(&mut array[pivot_index + 1..]);
}

fn partition<T: Ord>(array: &mut [T]) -> usize {
    let pivot_index = array.len() - 1;
    let mut i = 0;

    for j in 0..pivot_index {
        if array[j] <= array[pivot_index] {
            array.swap(i, j);
            i += 1;
        }
    }
    array.swap(i, pivot_index);
    i
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
	#[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
	#[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}