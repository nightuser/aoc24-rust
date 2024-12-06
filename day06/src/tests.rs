use super::{lower_bound, upper_bound};

#[test]
fn lower_bound_basic() {
    let xs = vec![2, 7, 7, 7, 10, 12];
    assert_eq!(lower_bound(&xs, &3), Some(&2));
    assert_eq!(lower_bound(&xs, &7), Some(&2));
    assert_eq!(lower_bound(&xs, &15), Some(&12));
    assert_eq!(lower_bound(&xs, &1), None);
    assert_eq!(lower_bound(&xs, &2), None);
}

#[test]
fn upper_bound_basic() {
    let xs = vec![2, 7, 7, 7, 10, 12];
    assert_eq!(upper_bound(&xs, &3), Some(&7));
    assert_eq!(upper_bound(&xs, &7), Some(&10));
    assert_eq!(upper_bound(&xs, &15), None);
    assert_eq!(upper_bound(&xs, &1), Some(&2));
    assert_eq!(upper_bound(&xs, &12), None);
}
