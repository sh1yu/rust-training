
#[test]
fn integration_sort_works() {
    let v = vec![2, 4, 3, 6, 5];
    let sorted_v = algori::sort(v);
    assert_eq!(sorted_v, vec![2, 3, 4, 5, 6]);
}

#[test]
fn integration_bubbo_sort_works() {
    let v = vec![2, 4, 3, 6, 5];
    let sorted_v = algori::algorithm::bubbo_sort(v);
    assert_eq!(sorted_v, vec![2, 3, 4, 5, 6]);
}

