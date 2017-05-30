use super::*;
use std::collections::HashSet;
use rulp::solver::Status;
use assert_approx_eq::*;

#[test]
fn simple_minimize_test() {
	let text_problem = "	
		# This is a problem;

		var a;
		var b;

		minimize objective: -2.*a + 5.*b;

		subject to constraint_1: a + b >= 10.;
		subject to constraint_2: b >= 5.;
		subject to constraint_3: a <= 15;

		# Nothing more to see here;
	";

	let mut builder = Builder::new();
	let lp = Parser::lp_from_text(text_problem, builder);
	println!("{}", lp);
	let solver = SimplexSolver::new(lp);
	let solution = solver.solve();
}

fn create_dummy_lp() -> Lp {
	let A = matrix![2., 1., 1., 0.;
					1., 2., 0., 1.];
	let b = vec![4., 3.];
	let c = vec![-1., -1., 0., 0.];
	let mut vars = HashSet::new();
	vars.insert("x1".to_string());
	vars.insert("x2".to_string());
	vars.insert("x3".to_string());
	vars.insert("x4".to_string());
	Lp {
			A: A,
			b: b,
			c: c,
			optimization: Optimization::Max,
			vars: vars,
			num_artificial_vars: 0
	}
}