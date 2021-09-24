use std::convert::TryInto;

pub fn sort<T: PartialOrd + Copy + Ord>(mut a: Vec<T>) {
    for i in 1..a.len() {
        let x = a[i]; // Copy value here
        let mut j = i;

        while j > 0 && a[j] > x {
            j = j - 1;
            a[j+1] = a[j];
        }

        a[j] = x
    }
}

#[cfg(test)]
mod test {
    use crate::sorting_algos::insertion_sort::sort;

    #[test]
    fn check_insert() {
        let mut array = vec![3,2,6,8,2,4,5,9,1,2,3,4,9,7];
        let mut arrayc: Vec::<i32> = Vec::new();
        assert_eq!(sort(array), arrayc.sort())
    }
}
