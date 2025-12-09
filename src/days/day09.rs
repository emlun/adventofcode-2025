// Solutions to Advent of Code 2025
// Copyright (C) 2025  Emil Lundberg <emil@emlun.se>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use crate::common::Solution;

#[derive(Eq, PartialEq)]
struct Point(i64, i64);
impl Point {
    fn area(&self, other: &Self) -> u64 {
        let Self(x1, y1) = self;
        let Self(x2, y2) = other;
        (1 + x1.abs_diff(*x2)) * (1 + y1.abs_diff(*y2))
    }
}

fn solve_a(points: &[Point]) -> u64 {
    points
        .iter()
        .enumerate()
        .flat_map(|(ip, p)| points.iter().skip(ip + 1).map(move |q| (p, q)))
        .map(|(p, q)| p.area(q))
        .max()
        .unwrap()
}

pub fn solve(lines: &[String]) -> Solution {
    let points: Vec<Point> = lines
        .iter()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            Point(x.parse().unwrap(), y.parse().unwrap())
        })
        .collect();

    (solve_a(&points).to_string(), "".to_string())
}
