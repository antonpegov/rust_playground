#[allow(dead_code)]
pub mod functions {
    fn largest_num(numbers: &[i32]) -> i32 {
        let mut largest = numbers[0];

        for &number in numbers {
            if number > largest {
                largest = number;
            }
        }

        largest
    }

    fn largest_char(list: &[char]) -> char {
        let mut largest = list[0];

        for &item in list {
            if &item > &largest {
                largest = item;
            }
        }

        largest
    }

    // generic function
    fn largest<T: std::cmp::PartialOrd + Copy>(items: &[T]) -> T {
        let mut largest = items[0];

        for &item in items {
            if item > largest {
                largest = item;
            }
        }

        largest
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn largest_test() {
            let numbers = vec![34, 50, 25, 100, 65];
            assert_eq!(largest(&numbers), 100);

            let chars = vec!['y', 'm', 'a', 'q'];
            assert_eq!(largest(&chars), 'y');
        }
    }
}
