pub struct Theme {
    pub background_image: Option<String>,
    pub background_image_opacity: f32,
    pub background_image_scale: f32,
    pub background_image_position: (f32, f32),
    pub background_image_blur: f32,
    pub background_image_brightness: f32,
    pub background_image_contrast: f32,
    pub background_color: u32,
    pub foreground_color: u32,
    pub text_color: u32,
    pub highlight_color: u32,
    pub border_color: u32,
}
