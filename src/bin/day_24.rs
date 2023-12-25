use std::time::Instant;

use aoc2023::read_input;
use nom::{IResult, character::complete::{u64 as pu64, i64 as pi64}, sequence::tuple, bytes::complete::tag, multi::separated_list1};
use z3::{SatResult, Solver, Context, Config, ast::Int};
#[derive(Debug, Clone, Copy)]
struct Ray {
    x: u64,
    y: u64,
    z: u64,
    xv: i64,
    yv: i64,
    zv: i64,
}

impl Ray {
    fn parse(input: &str) -> IResult<&str, Ray> {
        let (i, (x, _, y, _, z, _, xv, _, yv, _, zv)) = tuple(
            (pu64, tag(", "), pu64, tag(", "), pu64, tag(" @ "), pi64, tag(", "), pi64, tag(", "), pi64)
        )(input)?;
        Ok((i, Ray{
            x, y, z, xv, yv, zv
        }))
    }
}

fn main() {
    let start_time = Instant::now();
    let input = read_input("24");

    let (_, rays) = separated_list1.clone()(tag("\n"), Ray::parse)(&input).unwrap();

    let mut smt_string = String::new();
    smt_string.push_str(r#"
(declare-const ix Int)
(declare-const iy Int)
(declare-const iz Int)
(declare-const dx Int)
(declare-const dy Int)
(declare-const dz Int)
(declare-const t1 Int)
(declare-const t2 Int)
(declare-const t3 Int)
(declare-const t4 Int)
(declare-const t5 Int)
(declare-const Solution Int)
"#);
    for (i, ray) in rays.iter().take(5).enumerate() {
        smt_string.push_str(&format!("(assert (>= t{} 0))\n", i + 1));
        smt_string.push_str(&format!("(assert (= (+ (* t{} dx) ix) (+ (* {} t{}) {})))\n", i + 1, ray.xv, i + 1, ray.x));
        smt_string.push_str(&format!("(assert (= (+ (* t{} dy) iy) (+ (* {} t{}) {})))\n", i + 1, ray.yv, i + 1, ray.y));
        smt_string.push_str(&format!("(assert (= (+ (* t{} dz) iz) (+ (* {} t{}) {})))\n", i + 1, ray.zv, i + 1, ray.z));
    };
    smt_string.push_str(r#"
(assert (= (+ (+ ix iy) iz) Solution))
"#);

    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    let solver = Solver::new(&ctx);
    let solution = Int::new_const(&ctx, "Solution");

    solver.from_string(smt_string);
    match solver.check() {
        SatResult::Unsat => panic!("Unsat"),
        SatResult::Unknown => panic!("Unknown"),
        SatResult::Sat => {
            let model = solver.get_model().unwrap();
            let actual_solution = model.get_const_interp(&solution).unwrap();
            println!("{}", actual_solution);
            println!("Total time: {:?}", start_time.elapsed());
            return;
        },
    }
}
