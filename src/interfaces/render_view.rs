interface!(
    IRenderView,
    pub color_modulate[3](red: f32, green: f32, blue: f32) -> (),
    pub set_blend[4](blend: f32) -> ()
);
