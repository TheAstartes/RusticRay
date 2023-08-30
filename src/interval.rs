#[derive(Debug, Clone, Copy)]
pub struct Interval{
    pub min: f64,
    pub max: f64
}

impl Interval{
    pub fn new(min: f64, max: f64) -> Interval{
        Interval { min, max }
    }

    // fn contains(&self, x: f64) -> bool {
    //     self.min <= x && x <= self.max
    // }

    // fn surrounds(&self, x: f64) -> bool{
    //     self.min < x && x < self.max
    // }

    fn clamp(&self, x: f32) -> f64 {
        if self.min > x as f64 {return self.min}
        if self.max < x as f64 {return self.max}
    
        x as f64
    }

    pub fn sample(&self,color: f32, pixels_per_sample: i32) -> f64 {

        let scale = 1.0 / pixels_per_sample as f32;

        let new_color = color * scale;

        self.clamp(new_color)
    }
}
