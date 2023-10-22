use anyhow::Result;
use serde::Deserialize;
use lazy_static::lazy_static;
use std::{
    collections::HashMap,
    fs::read_to_string,
};
use input_linux::Key;


// Define the Button struct and any necessary implementations
#[derive(Debug)]
struct Button {
    label: String,
    action: Key,
}

impl Button {
    fn new_text(label: &str, action: Key) -> Self {
        Button {
            label: label.to_string(),
            action:action,
        }
    }
}



lazy_static! {
    static ref KEY_MAP: HashMap<&'static str, Key> = {
        let mut map = HashMap::new();
        map.insert("Esc", Key::Esc);
        map.insert("F1", Key::F1);
        map.insert("F2", Key::F2);
        map
    };
}

#[derive(Deserialize)]
struct ButtonConfig {
    label: String,
    key: String,
}

#[derive(Deserialize)]
struct PrimaryLayerButtonsConfig {
    buttons: Vec<ButtonConfig>,
}

#[derive(Deserialize)]
struct LayersConfig {
    primary_layer_buttons: PrimaryLayerButtonsConfig,
}

#[derive(Deserialize)]
struct Config {
    layers: LayersConfig,
}


impl Config {
    fn from_file(path: &str) -> Result<Self> {
        toml::from_str(&read_to_string(path)?).map_err(anyhow::Error::from)
    }
}

fn main() {

    let config_path = "/home/galder/git/rust/dictTest/etc/tiny-dfr.conf";
    let config = Config::from_file(config_path).unwrap();
    let key_map = HashMap::from([
        ("Esc", Key::Esc),
        ("F1", Key::F1),
        ("F2", Key::F2),
        ("F3", Key::F3),
        ("F4", Key::F4),
        ("F5", Key::F5),
        ("F6", Key::F6),
        ("F7", Key::F7),
        ("F8", Key::F8),
        ("F9", Key::F9),
        ("F10", Key::F10),
        ("F11", Key::F11),
        ("F12", Key::F12),
    ]);
    let to_find = ["Esc", "something"];
    for &key in &to_find {
        match key_map.get(key) {
            Some(value) => println!("Found key {}: with value {:?}", key, value ),
            None => println!("Could not find {key} in the map.")
        }
    }
    let mut primary_layer_buttons2 = Vec::new();
    for &key in &to_find {
        match KEY_MAP.get(key) {
            Some(value) =>  {
                primary_layer_buttons2.push(Button::new_text(key, *value));
                println!("Found key {}: with value {:?}", key, value );
            }
            None => println!("Could not find {key} in the map.")
        }
    }
    
    
    for (key, value) in &key_map {
        println!("iterate: key {}: with value {:?}", key, value );
    let mut primary_layer_buttons = Vec::new();
    for (key, value) in KEY_MAP.iter() {
        primary_layer_buttons.push(Button::new_text(key, *value));
        println!("Key: {}, Value: {:?}", key, value);
    }
    
    println!("Creating a vector from the config");
    let mut primary_layer_buttons = Vec::new();
    for button_config in &config.layers.primary_layer_buttons.buttons {
        let label = button_config.label.as_str();
        let key = &button_config.key.as_str();
        match key_map.get(key) {
            Some(value) => {
                primary_layer_buttons.push(Button::new_text(label, *value));
                println!("Found key {}: with value {:?}", label, value );
            }
            None => println!("Could not find {key} in the map.")
        }
    }
}
}
