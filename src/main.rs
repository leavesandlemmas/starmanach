use starmanach::math;

fn main() {
    let solver = math::root::Solver::new();
    
    let result = solver.solve(|x| {x*x - 2.0}, 0.0, 2.0);
    match result {
        Ok(sol) => {println!("{sol}")},
        Err(msg) => eprintln!("{msg}")
    }
}
