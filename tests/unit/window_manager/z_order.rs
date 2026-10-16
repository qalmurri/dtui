use super::ZOrder;

#[test]
fn insert_adds_window_to_top() {
    let mut z_order = ZOrder::default();

    z_order.insert(1);
    z_order.insert(2);

    assert_eq!(z_order.windows, vec![1, 2]);
}

#[test]
fn insert_existing_window_does_not_duplicate_it() {
    let mut z_order = ZOrder::default();

    z_order.insert(1);
    z_order.insert(2);
    z_order.insert(1);

    assert_eq!(z_order.windows, vec![2, 1]);
}

#[test]
fn raise_moves_window_to_top() {
    let mut z_order = ZOrder {
        windows: vec![1, 2, 3],
    };

    z_order.raise(1);

    assert_eq!(z_order.windows, vec![2, 3, 1]);
}

#[test]
fn raise_unknown_window_does_nothing() {
    let mut z_order = ZOrder {
        windows: vec![1, 2],
    };

    z_order.raise(99);

    assert_eq!(z_order.windows, vec![1, 2]);
}

#[test]
fn remove_deletes_window() {
    let mut z_order = ZOrder {
        windows: vec![1, 2, 3],
    };

    z_order.remove(2);

    assert_eq!(z_order.windows, vec![1, 3]);
}
