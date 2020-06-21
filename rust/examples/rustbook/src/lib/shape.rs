pub mod shape {
    #[derive(Default)]
    struct Polygon {
        vertices: Vec<(i32, i32)>,
        stroke_width: u8,
        fill: (u8, u8, u8),
        internal_id: String,
    }
}
