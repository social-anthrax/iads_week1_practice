pub fn sort<T: PartialOrd + Copy + Ord>(a: &mut Vec<T>) {
    for i in 1..a.len() {
        let x = a[i]; // Copy value here
        let mut j: isize = (i - 1) as isize;

        while j >= 0 && a[j as usize] > x {
            a[(j + 1) as usize] = a[j as usize];
            j = j - 1;
        }

        a[(j + 1) as usize] = x
    }
}

#[cfg(test)]
mod test {
    use crate::sorting_algos::insertion_sort::sort;

    #[test]
    fn check_insert() {
        let mut array = vec![3, 2, 6, 8, 2, 4, 5, 9, 1, 2, 3, 4, 9, 7];
        sort(&mut array);
        let mut new_array = array.to_vec();
        new_array.sort();
        assert_eq!(array, *new_array);
    }
}
