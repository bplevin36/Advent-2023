use std::time::Instant;

use aoc2023::read_input;
use float_eq::assert_float_eq;
use nom::{IResult, character::complete::{u64 as pu64, i64 as pi64}, sequence::tuple, bytes::complete::tag, multi::separated_list1};
use ndarray::prelude::*;
use ndarray_linalg::Solve;

#[derive(Debug)]
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

const MIN_XY: f64 = 200_000_000_000_000.;
const MAX_XY: f64 = 400_000_000_000_000.;

fn main() {
    let start_time = Instant::now();
    let input = read_input("24");

    let (_, rays) = separated_list1(tag("\n"), Ray::parse)(&input).unwrap();
    let mut num_intersections = 0;
    for i in 0..rays.len() {
        let ray = &rays[i];
        for j in i + 1..rays.len() {
            let other_ray = &rays[j];
            let a: Array2<f64> = array![
                [ray.xv as f64, -(other_ray.xv as f64)],
                [ray.yv as f64, -(other_ray.yv as f64)]];
            let b: Array1<f64> = array![(other_ray.x as i64 - ray.x as i64) as f64, (other_ray.y as i64 - ray.y as i64) as f64];
            match a.solve(&b) {
                Ok(x) => {
                    let ray_t = x[[0]];
                    let other_ray_t = x[[1]];
                    let point_x = ray.x as f64 + (ray.xv as f64 * ray_t);
                    let point_y = ray.y as f64 + (ray.yv as f64 * ray_t);
                    let other_point_x = other_ray.x as f64 + (other_ray.xv as f64 * other_ray_t);
                    let other_point_y = other_ray.y as f64 + (other_ray.yv as f64 * other_ray_t);

                    if ray_t < 0. || other_ray_t < 0. {
                        // paths intersect in the past
                        continue;
                    } else if point_x >= MIN_XY && point_x <= MAX_XY && point_y >= MIN_XY && point_y <= MAX_XY {
                        assert_float_eq!(point_x, other_point_x, rmax <= 2.);
                        assert_float_eq!(point_y, other_point_y, rmax <= 2.);
                        num_intersections += 1;
                    }
                },
                Err(_) => {
                    // no intersection
                    continue;
                },
            }
        }
    }
    println!("{}", num_intersections);
    println!("Total time: {:?}", start_time.elapsed());
}
