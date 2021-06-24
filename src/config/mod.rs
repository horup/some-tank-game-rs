use extensions::Ini;

pub struct Config {
    pub ini:Ini
}

impl Config {
    pub fn new(file:&str) -> Self{
        Self {
            ini:Ini::load_from_file(file).unwrap_or_default()
        }
    }

    fn key_bool_with_default(&self, key:&str, default:bool) -> bool {
        self.ini.get_from::<&str>(None, key).unwrap_or(if default {"true"} else {"false"}) == "true"
    }

    pub fn debug(&self) -> bool {
        self.key_bool_with_default("debug", false)
    }

    pub fn vsync(&self) -> bool {
        self.key_bool_with_default("vsync", true)
    }

    pub fn show_fps(&self) -> bool {
        self.key_bool_with_default("show_fps", false)
    }
}