use serde::export::fmt::Display;
use serde::export::Formatter;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize)]
pub struct Options {
    pub data: Data,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Data {
    pub dir: String,
}

impl Display for Options {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        writeln!(f, "data.dir={}", self.data.dir)
    }
}

pub fn get_or_create_config() -> Options {
    let home_dir = dirs::home_dir().expect("unable to determine home dir");
    let options_path = home_dir.join(".ranttrc");
    if options_path.exists() {
        load_config(&options_path)
    } else {
        let options = default_config(&home_dir);
        save_config(&options_path, &options);
        options
    }
}

pub fn load_config(options_path: &PathBuf) -> Options {
    let mut f = File::open(options_path).expect("Couldn't open config file");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("Couldn't read config file");
    toml::from_str(&contents).expect("Couldn't deserialize config")
}

pub fn save_config(options_path: &PathBuf, options: &Options) {
    let mut f = File::create(options_path).expect("couldn't open options_path for writing");
    write!(
        f,
        "{}",
        toml::to_string_pretty(options).expect("couldn't serialize config")
    )
    .unwrap();
}

fn default_config(home_dir: &PathBuf) -> Options {
    Options {
        data: Data {
            dir: String::from(
                home_dir
                    .join(".rantt")
                    .to_str()
                    .expect("couldn't convert options_path to str"),
            ),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_de() {
        let toml_str = r#"
            [data]
            dir = '/home/username/.rantt'
        "#;

        let options: Options = toml::from_str(toml_str).unwrap();
        assert_eq!("/home/username/.rantt", options.data.dir);
    }

    #[test]
    fn test_config_ser() {
        let options = Options {
            data: Data {
                dir: String::from("blah"),
            },
        };

        let toml_str = "[data]\ndir = 'blah'\n";
        assert_eq!(toml_str, toml::to_string_pretty(&options).unwrap());
    }
}
