pub mod algorithm;

pub fn sort<T: PartialOrd>(data: Vec<T>) -> Vec<T> {
    return algorithm::bubbo_sort(data);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sort_works() {
        let v = vec![2, 4, 3, 6, 5];
        let sorted_v = sort(v);
        assert_eq!(sorted_v, vec![2, 3, 4, 5, 6]);
    }
}
