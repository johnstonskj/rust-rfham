use lat_long::{Coordinate, Latitude, Longitude};
use rfham_maidenhead::{MaidenheadLocator, MaidenheadPrecision};
use std::str::FromStr;

#[test]
fn test_locator_point_to_string() {
    assert_eq!(
        "CN97hk",
        MaidenheadLocator::from_point_with_precision(
            Coordinate::new(
                Latitude::try_from(47.421375).unwrap(),
                Longitude::try_from(-121.410118).unwrap()
            ),
            MaidenheadPrecision::SubSquare
        )
        .unwrap()
        .as_ref()
    );
    assert_eq!(
        "CN97hk01",
        MaidenheadLocator::from_point_with_precision(
            Coordinate::new(
                Latitude::try_from(47.421375).unwrap(),
                Longitude::try_from(-121.410118).unwrap()
            ),
            MaidenheadPrecision::ExtendedSquare
        )
        .unwrap()
        .as_ref()
    );
}

#[test]
fn locator_string_to_point() {
    // SW corner of sub-square "hk": lat = 47 + 10/24, lon = -122 + 7/12
    assert_eq!(
        Coordinate::new(
            Latitude::try_from(47.0 + 10.0 / 24.0).unwrap(),
            Longitude::try_from(-122.0 + 7.0 / 12.0).unwrap()
        ),
        MaidenheadLocator::from_str("CN97hk")
            .unwrap()
            .to_point()
            .unwrap()
    )
}

#[test]
fn test_latlong_to_locator() {
    const DATA: &[(f64, f64, &str); 7] = &[
        (47.58877, -122.11236, "CN87WO61mh"),
        (47.58940972222223, -122.11284722222223, "CN87wo61kl"),
        (47.4375, -121.375, "CN97hk55aa"),
        (47.62048611111111, -122.34930555555556, "CN87TO88CW"), // Seattle Space Needle
        (46.786631944444444, -121.73402777777777, "CN96DS18WT"), // Paradise Inn, Mt. Rainier
        (47.953645833333326, -118.98993055555555, "DN07MW18FV"), // Grand Coulee Damn (main power house)
        (40.752604166666664, -73.97743055555556, "FN30as20rp"),  // Grand Central Station NYC
    ];
    for (lat, long, loc) in DATA {
        let location = Coordinate::new(
            Latitude::try_from(*lat).unwrap(),
            Longitude::try_from(*long).unwrap(),
        );
        let id = MaidenheadLocator::from_str(loc).unwrap();
        println!("assuring from_point_with_precision({location}) => {id}");
        assert_eq!(
            id,
            MaidenheadLocator::from_point_with_precision(
                location,
                MaidenheadPrecision::ExtendedSubSquare
            )
            .unwrap()
        );
    }
}

#[test]
fn test_locator_to_latlong() {
    const DATA: &[(&str, f64, f64); 7] = &[
        ("CN87WO61mh", 47.58871527777778, -122.11250000000001),
        ("CN87wo61kl", 47.58940972222223, -122.11319444444445),
        ("CN97hk55aa", 47.4375, -121.375),
        ("CN87TO88CW", 47.62048611111111, -122.34930555555556), // Seattle Space Needle
        ("CN96DS18WT", 46.786631944444444, -121.73402777777777), // Paradise Inn, Mt. Rainier
        ("DN07MW18FV", 47.953645833333326, -118.98993055555555), // Grand Coulee Damn (main power house)
        ("FN30as20rp", 40.752604166666664, -73.97743055555556),  // Grand Central Station NYC
    ];
    for (loc, lat, long) in DATA {
        let id = MaidenheadLocator::from_str(loc).unwrap();
        let location = Coordinate::new(
            Latitude::try_from(*lat).unwrap(),
            Longitude::try_from(*long).unwrap(),
        );
        println!("assuring {id}.to_point() => {location}");
        assert_eq!(location, id.to_point().unwrap());
    }
}
