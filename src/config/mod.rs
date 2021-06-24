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

    fn key_u32_with_default(&self, key:&str, default:u32) -> u32 {
        let v = self.ini.get_from::<&str>(None, key).unwrap_or("");
        if v == "" {
            return default;
        }

        v.parse::<u32>().unwrap_or(default)
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

    pub fn width(&self) -> u32 {
        self.key_u32_with_default("width", 1024)
    }

    pub fn height(&self) -> u32 {
        self.key_u32_with_default("height", 768)
    }

    pub fn fullscreen(&self) -> bool {
        self.key_bool_with_default("fullscreen", false)
    }
}