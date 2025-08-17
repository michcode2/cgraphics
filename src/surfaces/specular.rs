pub struct Specular {
    amount: f64,
}

impl Surface for Specular {
    fn get_value(&self, other: Rgba) -> Rgba {
        return other;
    }
}
