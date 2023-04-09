pub mod converter {
    use crate::dataStructures::{Vect2D,Vect3D};
    use std::f64::consts::PI;
    
    /*
        Function converts from the Position on a circle to the agles on a sphere projected onto the circle
        @param positionKartesian: Position on the circle
        @param cenerposition: Position of the center of the circle
        @param size: radius of the circle
        
        @return Vector containing the phi and theta angles where (0, 0) is at the center of the circle
     */
    pub fn PixelCoordinatesToSphereCoordinates(
        positionKartesian: Vect2D,
         centerposition: Vect2D,
          size: i16) -> Vect2D {

        let deltaX: f64 = positionKartesian.x - centerposition.x;
        let deltaY: f64 = positionKartesian.y - centerposition.y;
        let r: f64 = size as f64;
        let phi: f64 = (deltaX/r).asin();
        let theta: f64 = -(deltaY/r).asin();

        Vect2D {x: phi, y: theta}
    }

}
