
use starmanach::math;

fn main() {
    let solver = math::root::Solver{maxiter:50, atol:1e-15,rtol: 1e-16};
    let result = solver.solve(|e| {e - 0.99*e.sin() - 1.0}, 0.0, 2.0);
    match result {
        Ok(sol) => {println!("{sol}")},
        Err(msg) => eprintln!("{msg}")
    }
}
