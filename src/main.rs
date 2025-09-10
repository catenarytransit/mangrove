#![allow(unused_imports)]
use std::{fs::File, os::raw};
use std::io::prelude::*;

use gtfs_structures::*;

use prettytable::{*, Table, Row, Cell};



fn main() {
    // Create the table
    let mut table = Table::new();
    
    let output = File::create("test.csv").unwrap();
    let path = "test.zip";

    first_read(path);

    let data = extract_data(path);
    
    table.add_row(Row::from(data.trips.values().map(|a| a.route_id.clone()).filter(|a| !a.is_empty())));

    // Print the table to stdout
    let _ = table.to_csv(output);
    
    // Add a row per time
    table.add_row(row!["ABC", "DEFG", "HIJKLMN"]);
    table.add_row(row!["foobar", "bar", "foo"]);
}

pub fn first_read(path: &str) {
    let mut errors_found = Vec::new();
    let mut extras_incl = Vec::new();
    let raw_data_unwrapped = gtfs_structures::GtfsReader::default()
        .raw() // Won’t read shapes to save time and memory
        .read(path);
    if let Ok(raw_data) = raw_data_unwrapped {        
        if raw_data.stops.is_err() {
            errors_found.push(raw_data.stops.unwrap_err())
        }
        if raw_data.stop_times.is_err() {
            errors_found.push(raw_data.stop_times.unwrap_err())
        }
        if raw_data.routes.is_err() {
            errors_found.push(raw_data.routes.unwrap_err())
        }
        if raw_data.trips.is_err() {
            errors_found.push(raw_data.trips.unwrap_err())
        }
        if raw_data.agencies.is_err() {
            errors_found.push(raw_data.agencies.unwrap_err())
        }
        if raw_data.shapes.is_some() {
            extras_incl.push("Shapes")
        }
        if raw_data.fare_attributes.is_some() {
            extras_incl.push("Fare attributes")
        }
        if raw_data.fare_rules.is_some() {
            extras_incl.push("Fare rules")
        }
        if raw_data.fare_products.is_some() {
            extras_incl.push("Fare products")
        }
        if raw_data.fare_media.is_some() {
            extras_incl.push("Fare media")
        }
        if raw_data.rider_categories.is_some() {
            extras_incl.push("Ride Categories")
        }
        if raw_data.frequencies.is_some() {
            extras_incl.push("Frequencies")
        }
        if raw_data.feed_info.is_some() {
            extras_incl.push("Feed info ")
        }
        if raw_data.translations.is_some() {
            extras_incl.push("Translatons")
        }
        if raw_data.calendar.is_none() && raw_data.calendar_dates.is_none() {
            errors_found.push(gtfs_structures::Error::MissingFile("Calender info".to_string()))
        }

    }
    /*
    
    */
    else {
       errors_found.push(raw_data_unwrapped.unwrap_err());
    }
    println!("errors {errors_found:?}");
    println!("extras inc {extras_incl:?}");

}

pub fn extract_data(path: &str) -> Gtfs {
    let gtfs = gtfs_structures::GtfsReader::default()
        .read(path)
        .ok();
    gtfs.unwrap()
}