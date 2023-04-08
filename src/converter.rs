pub mod converter {
    use crate::dataStructures::{Vect2D,Vect3D};
    use std::f64::consts::PI;
    
    pub fn PixelCoordinatesToSphereCoordinates(positionKartesian: Vect2D, centerposition: Vect2D, size: i16) -> Vect2D {

        let deltaX: f64 = positionKartesian.x - centerposition.x;
        let deltaY: f64 = positionKartesian.y - centerposition.y;
        let r: f64 = size as f64 / 2.0;
        let phi: f64 = (deltaX/r).asin();
        let theta: f64 = -(deltaY/r).asin();

        Vect2D {x: phi, y: theta}
    }

}
