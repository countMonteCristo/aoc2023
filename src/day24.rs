use super::utils::Result;

type I = i128;

use mathru::{
    algebra::linear::{
        matrix::{General, Solve},
        vector::Vector,
    },
    matrix, vector,
};

#[derive(Debug)]
struct P3d {
    x: I,
    y: I,
    z: I,
}

#[derive(Debug)]
struct Hailstone {
    pos: P3d,
    v: P3d,
}

impl Hailstone {
    fn new(s: &str) -> Self {
        let parts = s
            .split(" ")
            .filter(|&s| s != "@" && !s.is_empty())
            .map(|x| x.trim_matches(',').parse::<I>().unwrap())
            .collect::<Vec<I>>();

        Self{
            pos: P3d { x: parts[0], y: parts[1], z: parts[2] },
            v: P3d { x: parts[3], y: parts[4], z: parts[5] }
        }
    }

    fn is_colinear_xy(&self, h: &Hailstone) -> bool {
        self.v.x * h.v.y == self.v.y * h.v.x
    }
}

fn intersects(h1: &Hailstone, h2: &Hailstone, x1: &f64, x2: &f64, y1: &f64, y2: &f64) -> bool {
    if h1.is_colinear_xy(h2) {
        h1.pos.x * h2.pos.y == h1.pos.y * h2.pos.x
    } else {
        let d = h2.v.x*h1.v.y - h1.v.x*h2.v.y;
        let d1 = h1.pos.x*h1.v.y - h1.pos.y*h1.v.x;
        let d2 = h2.pos.x*h2.v.y - h2.pos.y*h2.v.x;
        let q1 = d1*h2.v.x - d2*h1.v.x;
        let q2 = d1*h2.v.y - d2*h1.v.y;
        let x = (q1 as f64)/(d as f64);
        let y = (q2 as f64)/(d as f64);
        let t1 = (x - h1.pos.x as f64) / h1.v.x as f64;
        let t2 = (x - h2.pos.x as f64) / h2.v.x as f64;

        *x1 <= x && x <= *x2 &&  *y1 <= y && y <= *y2 && t1 >= 0. && t2 >= 0.

    }
}

fn solve1(hailstones: &Vec<Hailstone>) -> usize {
    let min = 200000000000000.;
    let max = 400000000000000.;
    // let min = 7.;
    // let max = 27.;
    let mut n = 0;
    for i in 0..hailstones.len()-1 {
        let h1 = &hailstones[i];
        for j in i+1..hailstones.len() {
            let h2 = &hailstones[j];
            if intersects(h1, h2, &min, &max, &min, &max) {
                n += 1;
            }
        }
    }

    n
}

fn solve2(h: &Vec<Hailstone>) -> I {
    let a: General<f64> = matrix![
        (h[1].v.y - h[0].v.y) as f64, (h[0].v.x - h[1].v.x) as f64, (h[0].pos.y - h[1].pos.y) as f64, (h[1].pos.x - h[0].pos.x) as f64;
        (h[2].v.y - h[0].v.y) as f64, (h[0].v.x - h[2].v.x) as f64, (h[0].pos.y - h[2].pos.y) as f64, (h[2].pos.x - h[0].pos.x) as f64;
        (h[3].v.y - h[0].v.y) as f64, (h[0].v.x - h[3].v.x) as f64, (h[0].pos.y - h[3].pos.y) as f64, (h[3].pos.x - h[0].pos.x) as f64;
        (h[4].v.y - h[0].v.y) as f64, (h[0].v.x - h[4].v.x) as f64, (h[0].pos.y - h[4].pos.y) as f64, (h[4].pos.x - h[0].pos.x) as f64
    ];

    let b: Vector<f64> = vector![
        ((h[1].pos.x * h[1].v.y - h[1].pos.y * h[1].v.x) - (h[0].pos.x * h[0].v.y - h[0].pos.y * h[0].v.x)) as f64;
        ((h[2].pos.x * h[2].v.y - h[2].pos.y * h[2].v.x) - (h[0].pos.x * h[0].v.y - h[0].pos.y * h[0].v.x)) as f64;
        ((h[3].pos.x * h[3].v.y - h[3].pos.y * h[3].v.x) - (h[0].pos.x * h[0].v.y - h[0].pos.y * h[0].v.x)) as f64;
        ((h[4].pos.x * h[4].v.y - h[4].pos.y * h[4].v.x) - (h[0].pos.x * h[0].v.y - h[0].pos.y * h[0].v.x)) as f64
    ];

    let c: General<f64> = matrix![
        (h[1].v.z - h[0].v.z) as f64, (h[0].v.y - h[1].v.y) as f64, (h[0].pos.z - h[1].pos.z) as f64, (h[1].pos.y - h[0].pos.y) as f64;
        (h[2].v.z - h[0].v.z) as f64, (h[0].v.y - h[2].v.y) as f64, (h[0].pos.z - h[2].pos.z) as f64, (h[2].pos.y - h[0].pos.y) as f64;
        (h[3].v.z - h[0].v.z) as f64, (h[0].v.y - h[3].v.y) as f64, (h[0].pos.z - h[3].pos.z) as f64, (h[3].pos.y - h[0].pos.y) as f64;
        (h[4].v.z - h[0].v.z) as f64, (h[0].v.y - h[4].v.y) as f64, (h[0].pos.z - h[4].pos.z) as f64, (h[4].pos.y - h[0].pos.y) as f64
    ];

    let d: Vector<f64> = vector![
        ((h[1].pos.y * h[1].v.z - h[1].pos.z * h[1].v.y) - (h[0].pos.y * h[0].v.z - h[0].pos.z * h[0].v.y)) as f64;
        ((h[2].pos.y * h[2].v.z - h[2].pos.z * h[2].v.y) - (h[0].pos.y * h[0].v.z - h[0].pos.z * h[0].v.y)) as f64;
        ((h[3].pos.y * h[3].v.z - h[3].pos.z * h[3].v.y) - (h[0].pos.y * h[0].v.z - h[0].pos.z * h[0].v.y)) as f64;
        ((h[4].pos.y * h[4].v.z - h[4].pos.z * h[4].v.y) - (h[0].pos.y * h[0].v.z - h[0].pos.z * h[0].v.y)) as f64
    ];

    let x1: Vector<f64> = a.solve(&b).unwrap();
    let x2: Vector<f64> = c.solve(&d).unwrap();

    (x1[0] + x1[1] + x2[1]) as I
}

pub fn run(data: &str, check: bool) -> Result {
    let hailstones = data.split('\n').map(|s| Hailstone::new(s)).collect();

    let ans1 = solve1(&hailstones);
    println!("Part1: {}", ans1);

    let ans2 = solve2(&hailstones);
    println!("Part2: {}", ans2);

    if !check || (ans1 == 15593 && ans2 == 757031940316991) {
        Ok(())
    } else {
        Err(())
    }
}
