/// bubbo_sort
/// sort given data using bubbo sort algorithm
/// param: Vec<i32> data
/// return: Vec<i32> sorted data
///
pub fn bubbo_sort<T: PartialOrd>(data: Vec<T>) -> Vec<T> {
    let mut sort_data = data;
    let length = sort_data.len();

    for i in 0..length - 1 {
        for j in 1..length - i {
            if sort_data.get(j - 1) > sort_data.get(j) {
                sort_data.swap(j - 1, j);
            }
        }
    }

    return sort_data;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bubbo_sort_works() {
        let v = vec![2, 4, 3, 6, 5];
        let sorted_v = bubbo_sort(v);
        assert_eq!(sorted_v, vec![2, 3, 4, 5, 6]);
    }
}
