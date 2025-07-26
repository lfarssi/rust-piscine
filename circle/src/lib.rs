use std::f32::consts::PI;
#[derive(Debug, Clone, Copy)]
pub struct Circle {
	pub center: Point,
	pub radius: f32,
}

impl Circle {
    pub fn new(x:f32, y:f32, radius:f32)->Self{
        Self{
            radius: radius,
            center:Point(x,y),
        }
    }
   pub fn diameter(&self)->f32{
        self.radius*2.0
   }
   pub fn area(&self)->f32{
    PI*self.radius*self.radius
   }
   pub fn intersect(&self, circle2:Circle)->bool{
    let dis = self.center.distance(circle2.center);
    dis< (self.radius+circle2.radius)
   }
}

#[derive(Debug, Clone, Copy)]
pub struct Point(pub f32,pub f32);

impl Point {
    pub fn distance(&self,circle2: Point)->f32{
        let x= self.0-circle2.0;
        let y= self.1-circle2.1;
        (x.powf(x)+y.powf(x)).sqrt()
    }
}