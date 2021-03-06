extern crate sort_lib;
// use sort_lib::quick_sort::{quick_sort, insertion_sort};

mod integration_tests {

    use sort_lib::quick_sort::quick_sort;
    use sort_lib::quick_sort::insertion_sort;

    #[test]
    fn test_insertion_sort() {
        let mut unordered_vec: Vec<isize> = vec![1, 9, 4, 7, 2, 6, 3, 5, 4, 5, 8];
        insertion_sort(&mut unordered_vec);

        assert_eq!(unordered_vec, vec![1, 2, 3, 4, 4, 5, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn test_quick_sort() {
        let mut unordered_vec: Vec<isize> = vec![1, 9, 4, 7, 2, 6, 3, 5, 4, 5, 8];
        quick_sort(&mut unordered_vec);

        assert_eq!(unordered_vec, vec![1, 2, 3, 4, 4, 5, 5, 6, 7, 8, 9]);
    }

}