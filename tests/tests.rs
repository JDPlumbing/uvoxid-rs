use uvoxid::{UvoxId, Delta};

#[test]
fn construct_and_unpack() {
    let pos = UvoxId::new(0, 1_000_000, 12345, -67890);
    let (frame, r, lat, lon) = pos.as_tuple();
    assert_eq!(frame, 0);
    assert_eq!(r, 1_000_000);
    assert_eq!(lat, 12345);
    assert_eq!(lon, -67890);
}

#[test]
fn apply_delta_increases_radius() {
    let mut pos = UvoxId::new(0, 1_000_000, 0, 0);
    let delta = Delta { dr_um: 100, dlat: 50, dlon: -50 };
    pos.apply_delta(delta);
    let (_, r, lat, lon) = pos.as_tuple();
    assert_eq!(r, 1_000_100);
    assert_eq!(lat, 50);
    assert_eq!(lon, -50);
}

#[test]
fn wrapping_lat_lon() {
    let mut pos = UvoxId::new(0, 0, i64::MAX, i64::MAX);
    let delta = Delta { dr_um: 0, dlat: 1, dlon: 1 };
    // should not panic
    pos.apply_delta(delta);
}

#[test]
fn cross_north_pole_wraps_correctly() {
    let mut pos = UvoxId::new(0, 0, 89_999_990, 0); // near north pole
    let delta = Delta { dr_um: 0, dlat: 20, dlon: 0 }; // push past pole
    pos.apply_delta(delta);

    let (_, _, lat, lon) = pos.as_tuple();
    // Expect latitude reflected back below pole
    assert!(lat <= 90_000_000);
    // Longitude should have shifted by ~180° if we crossed
    assert_eq!(lon % 180_000_000, 0);
}

#[test]
fn cross_south_pole_wraps_correctly() {
    let mut pos = UvoxId::new(0, 0, -89_999_990, 0); // near south pole
    let delta = Delta { dr_um: 0, dlat: -20, dlon: 0 }; // push past pole
    pos.apply_delta(delta);

    let (_, _, lat, lon) = pos.as_tuple();
    // Expect latitude reflected back above pole
    assert!(lat >= -90_000_000);
    // Longitude should have shifted by ~180° if we crossed
    assert_eq!(lon % 180_000_000, 0);
}
