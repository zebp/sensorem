use colored::{Color, Colorize};
use sensors::*;

fn main() {
    for chip in Sensors::new() {
        print_chip(chip).expect("unable to print chip");
    }
}

fn print_chip(chip: Chip) -> Result<(), LibsensorsError> {
    let chip_name = chip.get_name()?;
    let feature_pairs: Vec<_> = chip
        .into_iter()
        .filter_map(temperature_pair_for_feature)
        .collect();

    // Only display the sensors if we have some data to show.
    if feature_pairs.len() > 0 {
        println!("{}", chip_name);

        for (name, value) in feature_pairs {
            let color = color_for_temperature(value);
            let value_string = format!("{}°C", value).color(color);
            println!(" {}: {}", name, value_string);
        }
    }

    Ok(())
}

#[inline]
/// The text color to use for a given temperature.
fn color_for_temperature(input: f64) -> Color {
    match input {
        f if f > 85.0 => Color::BrightRed,
        f if f > 65.0 => Color::BrightYellow,
        f if f > 40.0 => Color::BrightGreen,
        f if f > 20.0 => Color::BrightBlue,
        f if f > 0.0 => Color::BrightMagenta,
        _ => Color::White,
    }
}

/// Gets the name and current temperature value for a given feature sensor.
fn temperature_pair_for_feature(feature: Feature) -> Option<(String, f64)> {
    let feature_name = feature.get_label().ok()?;
    let feature_value = feature
        .into_iter()
        .filter_map(|subfeature| match subfeature.subfeature_type() {
            SubfeatureType::SENSORS_SUBFEATURE_TEMP_INPUT => subfeature.get_value().ok(),
            _ => None,
        })
        .next();

    feature_value.map(|x| (feature_name, x))
}
