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

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Point {
    x: i64,
    y: i64,
}
impl Point {
    fn area(&self, other: &Self) -> u64 {
        (1 + self.x.abs_diff(other.x)) * (1 + self.y.abs_diff(other.y))
    }
}

#[derive(Debug)]
struct Line {
    p: Point,
    q: Point,
}

fn minmax<T: Ord>(a: T, b: T) -> (T, T) {
    if b < a {
        (b, a)
    } else {
        (a, b)
    }
}

fn intersects_interior(line: &Line, (rp, rq): (&Point, &Point)) -> bool {
    let (rminx, rmaxx) = minmax(rp.x, rq.x);
    let (rminy, rmaxy) = minmax(rp.y, rq.y);
    let (lminx, lmaxx) = minmax(line.p.x, line.q.x);
    let (lminy, lmaxy) = minmax(line.p.y, line.q.y);
    let rect_x = (rminx + 1)..rmaxx;
    let rect_y = (rminy + 1)..rmaxy;
    if line.p.x == line.q.x {
        rect_x.contains(&line.p.x) && !(lmaxy <= rminy || lminy >= rmaxy)
    } else {
        rect_y.contains(&line.p.y) && !(lmaxx <= rminx || lminx >= rmaxx)
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

fn solve_b(points: &[Point]) -> u64 {
    let lines: Vec<Line> = points
        .windows(2)
        .flat_map(|window| {
            if let [p, q] = window {
                Some(Line { p: *p, q: *q })
            } else {
                None
            }
        })
        .chain(std::iter::once(Line {
            p: points[points.len() - 1],
            q: points[0],
        }))
        .collect();

    points
        .iter()
        .enumerate()
        .flat_map(|(ip, p)| points.iter().skip(ip + 1).map(move |q| (p, q)))
        .fold(0, |max, (p, q)| {
            let a = p.area(q);
            if a > max && !lines.iter().any(|line| intersects_interior(line, (p, q))) {
                a
            } else {
                max
            }
        })
}

pub fn solve(lines: &[String]) -> Solution {
    let points: Vec<Point> = lines
        .iter()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            Point {
                x: x.parse().unwrap(),
                y: y.parse().unwrap(),
            }
        })
        .collect();

    debug_assert!(
        points.windows(3).all(|window| if let [p, _, r] = window {
            p.x != r.x && p.y != r.y
        } else {
            false
        }),
        "Expected no straight lines between triples of red tiles"
    );

    (solve_a(&points).to_string(), solve_b(&points).to_string())
}
