
use starmanach::math::quaternion::Quaternion;
fn main() {
//    let kepler =  kepler::KeplerEquation {ecc :  20.0} ;
//    let result = kepler.solve(5000.0);
//    match result {
//        Ok(sol) => {println!("{sol}")},
//        Err(msg) => eprintln!("{msg}")
//    }
    let q1 = Quaternion::i();
    let q2 = Quaternion::j();
    let q3 = Quaternion::k();    
    let q = q3 * (q1 + q2);
    let abs_q = q.abs();
    println!("|{q}| = {abs_q}");
    let qq = q * q.conj();
    
    println!("q q' = {qq}");
    let  qsq = q.abs_sq();
    println!("|q|^2 = {qsq}");
}
