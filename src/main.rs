
use starmanach::kepler;
fn main() {
    let kepler =  kepler::KeplerEquation {ecc :  20.0} ;
    let result = kepler.solve(5000.0);
    match result {
        Ok(sol) => {println!("{sol}")},
        Err(msg) => eprintln!("{msg}")
    }
}
