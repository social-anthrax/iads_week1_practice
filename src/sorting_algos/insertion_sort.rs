pub fn sort<T: PartialOrd + Copy + Ord>(a: &mut Vec<T>) {
    for i in 1..a.len() {
        let x = a[i];

        let mut loc = i; //don't decrease by one as last line doesn't increase.
        for j in (0..i).rev() {
            if a[j] <= x {break;}
            
            a[(j + 1)] = a[j];
            loc = j;
        }
        a[loc] = x;
    }
}

pub fn sort2<T: PartialOrd + Copy + Ord>(a: &mut Vec<T>) {
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
    use crate::sorting_algos::insertion_sort::{sort, sort2};

    #[test]
    fn check_insert() {
        let mut array = vec![3, 2, 6, 8, 2, 4, 5, 9, 1, 2, 3, 4, 9, 7];
        let mut new_array = array.to_vec();
        let mut new_array2 = new_array.to_vec();
        sort(&mut array);
        sort2(&mut new_array2);
        new_array.sort();

        assert_eq!(array, *new_array);
        assert_eq!(new_array, new_array2);
    }
}
