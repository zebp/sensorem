use std::{io::Write, time::Duration};

use clap::{App, Arg};
use colored::{Color, Colorize};
use crossterm::{
    cursor::MoveTo,
    terminal::{Clear, ClearType},
};
use sensors::*;

fn main() {
    let matches = App::new("sensorem")
        .version("1.0.0")
        .author("Zeb Piasecki <zeb@zebulon.dev>")
        .about("Colorful sensors")
        .arg(
            Arg::with_name("watch")
                .short("w")
                .long("watch")
                .value_name("INTERVAL")
                .help("Watch the sensors")
                .takes_value(true),
        )
        .get_matches();

    // How often, if at all, we should display the sensor data.
    let watch_interval: Option<f32> = matches
        .value_of("watch")
        .map(|s| s.parse().expect("watch interval must be a number"));

    if let Some(watch_interval) = watch_interval {
        let watch_interval = Duration::from_secs_f32(watch_interval);

        loop {
            // Clear the screen and go to the top left.
            print!("{}{}", Clear(ClearType::All), MoveTo(0, 0));

            print_chips();

            // Wait for the next interval.
            std::thread::sleep(watch_interval);
        }
    } else {
        print_chips();
    }
}

/// Prints the chips to the terminal and buffers the output so the user doesn't see partial output
/// if they are running in watch mode.
fn print_chips() {
    let mut buffer = Vec::new();

    for chip in Sensors::new() {
        write_chip(chip, &mut buffer).expect("unable to print chip");
    }

    // Assured to be valid UTF-8 because we are writing our selves.
    print!("{}", String::from_utf8(buffer).unwrap());
}

/// Writes the temperature information about the given chip to the output buffer.
fn write_chip(chip: Chip, fmt: &mut dyn Write) -> anyhow::Result<()> {
    let chip_name = chip.get_name()?;
    let feature_pairs: Vec<_> = chip
        .into_iter()
        .filter_map(temperature_pair_for_feature)
        .collect();

    // Only display the sensors if we have some data to show.
    if !feature_pairs.is_empty() {
        writeln!(fmt, "{}", chip_name)?;

        for (name, value) in feature_pairs {
            let color = color_for_temperature(value);
            let value_string = format!("{}°C", value).color(color);
            writeln!(fmt, " {}: {}", name, value_string)?;
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
    let feature_value =
        feature
            .into_iter()
            .find_map(|subfeature| match subfeature.subfeature_type() {
                SubfeatureType::SENSORS_SUBFEATURE_TEMP_INPUT => subfeature.get_value().ok(),
                _ => None,
            });

    feature_value.map(|x| (feature_name, x))
}
