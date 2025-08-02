    pub mod areas_volumes; 

    // use areas_volumes::{circle_area, square_area, rectangle_area, triangle_area};
    pub use crate::areas_volumes::*;
    pub fn area_fit(
        (x, y): (usize, usize), // x== length, y==width
        kind: GeometricalShapes, //circle or square or rec or triangle
        times: usize, //
        (a, b): (usize, usize),
    ) -> bool {
        match kind {
        GeometricalShapes::Circle =>{
                let circle_a=circle_area(a)*times as f64;
                let rec_area = x*y;
                if circle_a<= rec_area as f64{
                    return true;
                } else{
                    return false;
                }
            },
            GeometricalShapes::Square  =>{
                let square_a=square_area(a)*times;
                let rec_area = x*y;
                if square_a<= rec_area{
                    return true;
                } else{
                    return false;
                }
            },
            GeometricalShapes::Rectangle  =>{
                let rec_areas=rectangle_area(a,b)*times;
                let rec_area = x*y;
                if rec_areas<= rec_area{
                    return true;
                } else{
                    return false;
                }
            },
            GeometricalShapes::Triangle =>{
                let triagnel_a=triangle_area(a,b)*times as f64;
                let rec_area = x*y;
                if triagnel_a<= rec_area as f64{
                    return true;
                } else{
                    return false;
                }
            },
        }
    }

pub fn volume_fit(
    (x, y, z): (usize, usize, usize),
    kind: areas_volumes::GeometricalVolumes,
    times: usize,
    (a, b, c): (usize, usize, usize),
) -> bool {
     match kind {
        GeometricalVolumes::Sphere =>{
                let circle_a=sphere_volume(a)*times as f64;
                let box_a = x*y*z;
                if circle_a<= box_a as f64{
                    return true;
                } else{
                    return false;
                }
            },
            GeometricalVolumes::Cube  =>{
                let cube_a=cube_volume(a)*times;
                let box_a = x*y*z;
                if cube_a<= box_a{
                    return true;
                } else{
                    return false;
                }
            },
            GeometricalVolumes::Cone  =>{
                let con_a=cone_volume(a,b)*times as f64;
                let box_a = x*y*z;
                if con_a<= box_a as f64{
                    return true;
                } else{
                    return false;
                }
            },
            GeometricalVolumes::TriangularPyramid =>{
                let triagnel_a=triangular_pyramid_volume(a as f64,b)*times as f64;
                let box_a = x*y*z;
                if triagnel_a<= box_a as f64{
                    return true;
                } else{
                    return false;
                }
            },
            GeometricalVolumes::Parallelepiped =>{
                let parraa_a=parallelepiped_volume(a,b,c)*times ;
                let box_a = x*y*z;
                if parraa_a<= box_a {
                    return true;
                } else{
                    return false;
                }
            },
        }
}