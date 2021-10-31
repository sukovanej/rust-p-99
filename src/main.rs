#![allow(dead_code)]

fn main() {
    println!("Hello, world!");
}

mod p01 {
    fn my_last<T>(input: &Vec<T>) -> Option<&T> {
        if input.len() == 0 {
            None
        } else {
            input.get(input.len() - 1)
        }
    }

    #[test]
    fn test_my_last() {
        let parametrize: Vec<(Vec<i32>, Option<&i32>)> = vec![(vec![], None), (vec![1], Some(&1))];

        for (input, expected) in parametrize.into_iter() {
            assert_eq!(my_last(&input), expected);
        }
    }
}

mod p02 {
    fn my_but_last<T>(input: &Vec<T>) -> Option<&T> {
        if input.len() <= 1 {
            None
        } else {
            input.get(input.len() - 2)
        }
    }

    #[test]
    fn test_my_but_last() {
        let parametrize: Vec<(Vec<i32>, Option<&i32>)> = vec![
            (vec![], None),
            (vec![1], None),
            (vec![1, 2], Some(&1)),
            (vec![1, 2, 3], Some(&2)),
        ];

        for (input, expected) in parametrize.into_iter() {
            assert_eq!(my_but_last(&input), expected);
        }
    }
}

mod p03 {
    fn my_kth_element<T>(input: &Vec<T>, k: usize) -> Option<&T> {
        if input.len() < k {
            None
        } else {
            input.get(k - 1)
        }
    }

    #[test]
    fn test_my_kth_element() {
        let parametrize: Vec<(Vec<i32>, usize, Option<&i32>)> = vec![
            (vec![], 1, None),
            (vec![1], 2, None),
            (vec![1, 2], 1, Some(&1)),
            (vec![1, 2, 3], 2, Some(&2)),
        ];

        for (input, k, expected) in parametrize.into_iter() {
            assert_eq!(my_kth_element(&input, k), expected);
        }
    }
}

mod p04 {
    fn my_list_size<T>(input: &Vec<T>) -> usize {
        return input.len();
    }

    #[test]
    fn test_my_list_size() {
        let parametrize: Vec<(Vec<i32>, usize)> = vec![
            (vec![], 0),
            (vec![1], 1),
            (vec![1, 2], 2),
            (vec![1, 2, 3], 3),
        ];

        for (input, expected) in parametrize.into_iter() {
            assert_eq!(my_list_size(&input), expected);
        }
    }
}

mod p05 {
    fn my_reverse_list<T: Clone>(input: &Vec<T>) -> Vec<T> {
        let mut new_list = input.clone();
        new_list.reverse();
        new_list
    }

    #[test]
    fn test_my_list_size() {
        let parametrize: Vec<(Vec<i32>, Vec<i32>)> = vec![
            (vec![], vec![]),
            (vec![1], vec![1]),
            (vec![1, 2], vec![2, 1]),
            (vec![1, 2, 3], vec![3, 2, 1]),
        ];

        for (input, expected) in parametrize.into_iter() {
            assert_eq!(my_reverse_list(&input), expected);
        }
    }
}

mod p06 {
    fn my_is_lindrome<T: Eq>(input: &Vec<T>) -> bool {
        let center_index = input.len() / 2;

        for (i, (left, right)) in input.iter().zip(input.iter().rev()).enumerate() {
            if i > center_index {
                break;
            }

            if left != right {
                return false;
            }
        }

        return true;
    }

    #[test]
    fn test_my_is_palindrome() {
        let parametrize: Vec<(Vec<i32>, bool)> = vec![
            (vec![], true),
            (vec![1], true),
            (vec![1, 2], false),
            (vec![1, 2, 3], false),
            (vec![1, 2, 1], true),
            (vec![1, 2, 2, 1], true),
            (vec![1, 2, 2, 2, 1], true),
            (vec![1, 2, 2, 2, 1, 1], false),
        ];

        for (input, expected) in parametrize.into_iter() {
            assert_eq!(my_is_lindrome(&input), expected);
        }
    }
}

mod p07 {
    enum VecOrItem<T> {
        Item(T),
        NestedItem(Vec<VecOrItem<T>>),
    }

    fn my_flatten<T: Clone>(input: &Vec<VecOrItem<T>>) -> Vec<T> {
        let mut result = Vec::<T>::new();

        for i in input.iter() {
            match i {
                VecOrItem::Item(t) => result.push(t.clone()),
                VecOrItem::NestedItem(v) => result.append(&mut my_flatten(v)),
            }
        }

        result
    }

    fn test_my_flatten() {
        let parametrize: Vec<(Vec<VecOrItem<i32>>, Vec<i32>)> = vec![
            (vec![], vec![]),
            (vec![VecOrItem::Item(1), VecOrItem::Item(2)], vec![1, 2]),
            (
                vec![
                    VecOrItem::Item(1),
                    VecOrItem::Item(2),
                    VecOrItem::NestedItem(vec![VecOrItem::Item(3)]),
                ],
                vec![1, 2, 3],
            ),
            (
                vec![
                    VecOrItem::Item(1),
                    VecOrItem::NestedItem(vec![VecOrItem::Item(2), VecOrItem::Item(3)]),
                    VecOrItem::Item(4),
                    VecOrItem::NestedItem(vec![
                        VecOrItem::Item(5),
                        VecOrItem::NestedItem(vec![VecOrItem::Item(6), VecOrItem::Item(7)]),
                    ]),
                ],
                vec![1, 2, 3, 4, 5, 6, 7],
            ),
        ];

        for (input, expected) in parametrize.into_iter() {
            assert_eq!(my_flatten(&input), expected);
        }
    }
}

mod p08 {
    fn my_elimite_consecutive_duplicates<T, U>(input: T) -> Iterator<U>
    where 
        T: IntoIterator, 
        T::Item: U
        {
        
    }

    fn test_my_elimite_consecutive_duplicates() {
        let parametrize: Vec<(&str, &str)> = vec![
            ("", ""),
        ];

        for (input, expected) in parametrize.into_iter() {
            assert_eq!(my_elimite_consecutive_duplicates(&input), expected);
        }
    }
}
