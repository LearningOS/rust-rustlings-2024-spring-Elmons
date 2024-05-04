/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

fn sort<T: PartialOrd>(array: &mut [T]) {
	//TODO
    quick_sort(array, 0, array.len() - 1);
}

fn quick_sort<T: PartialOrd>(a: &mut [T], s: usize, e: usize) {
    if s >= e {
        return;
    }

    let (mut l, mut r) = (s, e);
    while l < r {
        while (l < r && a[l] <= a[r]) {
            r -= 1;
        }
        a.swap(l, r);
        if (l < r) {
            l += 1;
        }

        while (l < r && a[l] <= a[r]) {
            l += 1;
        }
        a.swap(l, r);
        if (l < r) {
            r -= 1;
        }
    }
    quick_sort(a, s, l);
    quick_sort(a, l + 1, e);
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