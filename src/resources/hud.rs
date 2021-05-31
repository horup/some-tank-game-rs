pub struct Hud {
    pub top_left_text:String,
    pub top_right_text:String,
    pub center_text:String
}

impl Default for Hud {
    fn default() -> Self {
        Self {
            top_left_text:"".into(),
            top_right_text:"".into(),
            center_text:"".into()
        }
    }
}