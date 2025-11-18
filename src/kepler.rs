use crate::math::real::Real;              
         


pub struct KeplerEquation {
    ecc : Real,
} 

impl KeplerEquation {
    pub fn from_eccentricity(ecc : Real) -> Self{
        Self {ecc}
    }

    pub fn mean_anomaly(&self, eccentric_anomaly: Real) -> Real {
        if self.ecc <= 1.0 {
            eccentric_anomaly - self.ecc * eccentric_anomaly.sin()    
        } else {
            self.ecc * eccentric_anomaly.sinh() - eccentric_anomaly 
        }
    } 

    pub fn eccentric_anomaly(&self, mean_anomaly: Real) -> Real {
        use crate::math::root::Solver;
        use crate::math::real::PI;              
        let solver = Solver::new();
        let f = |x|{ self.mean_anomaly(x) - mean_anomaly};
        let sol = solver.solve(f, 0.0, 2.0 * PI).expect("Shouldn't fail.");
        sol.root
     } 

    
}
