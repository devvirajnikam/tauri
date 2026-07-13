#[derive(Debug, Clone, PartialEq, Default)]
struct Settings {
    dark_mode: bool,
    font_size: u8,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    Up,
    Down,
}

pub fn run() {
    println!("\n17. Derived traits");

    let default_settings = Settings::default();
    let custom_settings = Settings {
        dark_mode: true,
        font_size: 16,
    };
    let cloned_settings = custom_settings.clone();

    println!("Default settings: {:?}", default_settings);
    println!("Custom settings: {:?}", custom_settings);
    println!("Cloned settings: {:?}", cloned_settings);
    println!("Settings equal: {}", custom_settings == cloned_settings);

    let direction = Direction::Up;
    let copied_direction = direction;

    println!("Direction still usable after copy: {:?}", direction);
    println!("Copied direction: {:?}", copied_direction);

    let down = Direction::Down;
    println!("Another direction variant: {:?}", down);
}
