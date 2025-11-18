use crate::math::real::Real;              
use crate::math::root;         
use crate::math::real::PI; 

pub struct KeplerEquation {
    pub ecc : Real,
} 

impl KeplerEquation {
   
    pub fn mean_anomaly(&self, eccentric_anomaly: Real) -> Real {
        if self.ecc == 0.0 {
            return eccentric_anomaly;
        }
        if self.ecc <= 1.0 {
            eccentric_anomaly - self.ecc * eccentric_anomaly.sin()    
        } else {
            self.ecc * eccentric_anomaly.sinh() - eccentric_anomaly 
        }
    }

    pub fn eccentric_anomaly(&self, mean_anomaly: Real) -> Real {
        let sol = self.solve(mean_anomaly).expect("Root solver reports unexpected error.");
        sol.root    
    } 


    fn jac(&self, eccentric_anomaly : Real) -> Real {
        if self.ecc <= 1.0 {
            1.0 - self.ecc * eccentric_anomaly.cos()    
        } else {
            self.ecc * eccentric_anomaly.cosh() - 1.0 
        }

    } 

    pub fn solve(&self, mean_anomaly: Real) -> Result<root::Solution, root::SolveError> {  
             
        let solver = root::Solver::new(); 
        let f = |x|{ self.mean_anomaly(x) - mean_anomaly};
        let df = |x| {self.jac(x)};
        let (a,b) = self.starting_bracket(mean_anomaly);
        solver.solve_with_jac(f, df, a, b)
     } 
    
    fn starting_bracket(&self, mean_anomaly: Real) -> (Real, Real) {
        if self.ecc <= 1.0 { self.elliptic_start(mean_anomaly)
        } else {self.hyperbolic_start(mean_anomaly)}
    }
    
    fn elliptic_start(&self, mean_anomaly: Real) ->  (Real, Real) {
                            
            if mean_anomaly <= PI {
               (0.0, PI)        
            } else { 
               (PI, 2.0*PI)
            }    
    }
    
    fn hyperbolic_start(&self,  mean_anomaly: Real) -> (Real, Real) {

        let a = Real::asinh(mean_anomaly / self.ecc);   
            let p = 6.0 * (1.0 - 1.0/self.ecc);
            let q = -6.0/self.ecc * mean_anomaly;
            let b = root_of_depressed_cubic(p, q); 
           (a, b)
          
    } 
}

    fn root_of_depressed_cubic(p : Real, q : Real) -> Real {
        let disc = q.powi(2) / 4.0 + p.powi(3) / 27.0;
        let sq_disc = disc.sqrt();
        let u1 = -q/2.0 + sq_disc;
        let u2 = -q/2.0 - sq_disc;
        u1.cbrt() + u2.cbrt()        
    }   
