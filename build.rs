fn main() {
    println!("cargo::rerun-if-changed=fonts/menu-builder.toml");
    iced_fontello::build("fonts/menu-builder.toml").expect("Build menu-builder font");
}

