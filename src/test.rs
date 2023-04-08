#[cfg(test)]
mod tests {
    use std::f64::consts::PI;
    use crate::converter::converter::PixelCoordinatesToSphereCoordinates;
    use crate::dataStructures::Vect2D;
    #[test]
    fn test_10_0() {
        assert_eq!(
            PixelCoordinatesToSphereCoordinates(Vect2D{x: 10.0, y: 0.0}, Vect2D{x: 0.0, y:0.0}, 20i16), 
            Vect2D{ x: PI/2.0, y: 0.0}
        );
    }
    #[test]
    fn test_0_10() {
        assert_eq!(
            PixelCoordinatesToSphereCoordinates(Vect2D{x: 0.0, y: 10.0}, Vect2D{x: 0.0, y:0.0 }, 20i16), 
            Vect2D{x: 0.0, y: -PI/2.0}
        );
    }
    #[test]
    fn test_m10_0() {
        assert_eq!(
            PixelCoordinatesToSphereCoordinates(Vect2D{x: -10.0, y: 0.0}, Vect2D{x: 0.0, y:0.0 }, 20i16), 
            Vect2D{ x: -PI/2.0, y: 0.0}
        );
    }
    #[test]
    fn test_0_m10() {
        assert_eq!(
            PixelCoordinatesToSphereCoordinates(Vect2D{x: 0.0, y: -10.0}, Vect2D{x: 0.0, y:0.0 }, 20i16), 
            Vect2D{x:0.0, y: PI/2.0}
        );
    }
    #[test]
    fn test_0_0() {
        assert_eq!(
            PixelCoordinatesToSphereCoordinates(Vect2D{x: 0.0, y: 0.0}, Vect2D{x: 0.0, y:0.0 }, 20i16), 
            Vect2D{x:0.0, y: 0.0}
        );
    }
}
