use std::f64::consts::PI;
#[derive(Debug, Clone, Copy)]
pub struct Circle {
	pub center: Point,
	pub radius: f64,
}

impl Circle {
    pub fn new(x:f64, y:f64, radius:f64)->Self{
        Self{
            radius: radius,
            center:Point(x,y),
        }
    }
   pub fn diameter(&self)->f64{
        self.radius*2.0
   }
   pub fn area(&self)->f64{
    PI*self.radius*self.radius
   }
   pub fn intersect(&self, circle2:Circle)->bool{
    let dis = self.center.distance(circle2.center);
    dis< (self.radius+circle2.radius)
   }
}

#[derive(Debug, Clone, Copy)]
pub struct Point(pub f64,pub f64);

impl Point {
    pub fn distance(&self,circle2: Point)->f64{
        let x= self.0-circle2.0;
        let y= self.1-circle2.1;
        (x.powi(2)+y.powi(2)).sqrt()
    }
}