use std::str::FromStr;

use lat_long::{Coordinate, Latitude, Longitude};
use rfham_geo::grid::maidenhead::MaidenheadLocator;

fn test_data() -> impl Iterator<Item = (Coordinate, MaidenheadLocator)> {
    const DATA: &[(f64, f64, &str)] = &[(47.4375868, -121.374826, "CN97hk")];
    DATA.iter().map(|(lat, long, id)| {
        (
            Coordinate::new(
                Latitude::try_from(*lat).unwrap(),
                Longitude::try_from(*long).unwrap(),
            ),
            MaidenheadLocator::from_str(id).unwrap(),
        )
    })
}

#[test]
fn test_latlong_to_locator() {
    for (location, id) in test_data() {
        assert_eq!(id, MaidenheadLocator::from(location));
    }
}

#[test]
fn test_locator_to_latlong() {
    for (location, id) in test_data() {
        assert_eq!(location, id.to_point().unwrap());
    }
}
