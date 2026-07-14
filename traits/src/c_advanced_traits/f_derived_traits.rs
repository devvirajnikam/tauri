// Scenario:
// A type needs common behavior like debug printing, cloning, equality checks, or
// default values.
//
// Thinking:
// derive asks the compiler to implement common traits automatically.

#[derive(Debug, Clone, PartialEq, Default)]
struct Settings {
    dark_mode: bool,
    font_size: u8,
}

pub fn run() {
    println!("\n15. Derived traits");

    let settings = Settings {
        dark_mode: true,
        font_size: 16,
    };
    let cloned = settings.clone();
    let default_settings = Settings::default();

    println!("Settings: {:?}", settings);
    println!(
        "Settings fields -> dark_mode: {}, font_size: {}",
        settings.dark_mode, settings.font_size
    );
    println!("Cloned equals original: {}", cloned == settings);
    println!("Default settings: {:?}", default_settings);
}
