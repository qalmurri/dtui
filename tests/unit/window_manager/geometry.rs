use super::Geometry;

#[test]
fn cascade_keeps_window_inside_terminal() {
    let geometry = Geometry::cascade(3, 100, 30);

    assert!(geometry.x + geometry.width <= 100);
    assert!(geometry.y + geometry.height <= 30);
}

#[test]
fn cascade_adapts_to_small_terminal() {
    let geometry = Geometry::cascade(0, 15, 4);

    assert_eq!(
        geometry,
        Geometry {
            x: 0,
            y: 0,
            width: 15,
            height: 4,
        }
    );
}

#[test]
fn fit_clamps_position_and_size() {
    let geometry = Geometry {
        x: 90,
        y: 40,
        width: 50,
        height: 20,
    }
    .fit(100, 30);

    assert_eq!(
        geometry,
        Geometry {
            x: 50,
            y: 10,
            width: 50,
            height: 20,
        }
    );
}

#[test]
fn resize_respects_minimum_size() {
    let geometry = Geometry {
        x: 0,
        y: 0,
        width: 30,
        height: 10,
    }
    .resize(-100, -100, 100, 30);

    assert_eq!(geometry.width, 20);
    assert_eq!(geometry.height, 5);
}

#[test]
fn resize_does_not_exceed_available_area() {
    let geometry = Geometry {
        x: 10,
        y: 5,
        width: 30,
        height: 10,
    }
    .resize(100, 100, 60, 25);

    assert_eq!(geometry.width, 60);
    assert_eq!(geometry.height, 25);
    assert_eq!(geometry.x, 0);
    assert_eq!(geometry.y, 0);
    assert!(geometry.x + geometry.width <= 60);
    assert!(geometry.y + geometry.height <= 25);
}

#[test]
fn resize_shifts_window_when_right_edge_is_reached() {
    let geometry = Geometry {
        x: 70,
        y: 0,
        width: 30,
        height: 10,
    }
    .resize(10, 0, 100, 30);

    assert_eq!(geometry.width, 40);
    assert_eq!(geometry.x, 60);
    assert_eq!(geometry.x + geometry.width, 100);
}

#[test]
fn resize_shifts_window_when_bottom_edge_is_reached() {
    let geometry = Geometry {
        x: 0,
        y: 20,
        width: 30,
        height: 10,
    }
    .resize(0, 5, 100, 30);

    assert_eq!(geometry.height, 15);
    assert_eq!(geometry.y, 15);
    assert_eq!(geometry.y + geometry.height, 30);
}

#[test]
fn resize_handles_zero_sized_terminal() {
    let geometry = Geometry {
        x: 10,
        y: 5,
        width: 30,
        height: 10,
    }
    .resize(1, 1, 0, 0);

    assert_eq!(
        geometry,
        Geometry {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        }
    );
}
