#[derive(Debug, Clone)]
pub struct ClickedPoints {
    pub points: Vec<(f32, f32)>,
}

impl ClickedPoints {
    pub fn new() -> Self {
        Self { points: vec![] }
    }
    pub fn add(&mut self, x: f32, y: f32) {
        self.points.push((x, y));
    }
}

pub fn chaikins(original: ClickedPoints) -> ClickedPoints {
    /*
    if points.len() < 2 {
        return points;
    }
    */
    let mut smoothed: ClickedPoints = ClickedPoints::new();

    smoothed.add(original.points[0].0, original.points[0].1);

    for i in 0..original.points.len() - 1 {
        let p1 = &original.points[i];
        let p2 = &original.points[i + 1];
        let p3 = 
            (0.75*p1.0 + 0.25*p2.0,
            0.75*p1.1 + 0.25*p2.1);
        let p4 = 
            (0.25*p1.0 + 0.75*p2.0,
            0.25*p1.1 + 0.75*p2.1);
     
        smoothed.add(p3.0, p3.1);
        smoothed.add(p4.0, p4.1);
    }
    smoothed.add(original.points[original.points.len() - 1].0, original.points[original.points.len() - 1].1);
    
    smoothed
}