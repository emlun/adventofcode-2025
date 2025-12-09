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

use std::ops::Range;

use crate::{common::Solution, util::iter::Countable};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Point {
    x: i64,
    y: i64,
}
impl Point {
    fn area(&self, other: &Self) -> u64 {
        (1 + self.x.abs_diff(other.x)) * (1 + self.y.abs_diff(other.y))
    }

    fn polarity(&self, i: usize, l: &Self, r: &Self) -> bool {
        ((((self.x - l.x).signum() + (self.y - l.y).signum())
            * ((r.x - self.x).signum() + (r.y - self.y).signum()))
            > 0)
            == i.is_multiple_of(2)
    }
}

#[derive(Debug)]
struct Line {
    p: Point,
    q: Point,
}

fn interval_interior(a: i64, b: i64) -> Range<i64> {
    (std::cmp::min(a, b) + 1)..std::cmp::max(a, b)
}

fn minmax<T: Ord>(a: T, b: T) -> (T, T) {
    if b < a {
        (b, a)
    } else {
        (a, b)
    }
}

fn intersects_interior(line: &Line, (rp, rq): (&Point, &Point)) -> bool {
    let rect_x = interval_interior(rp.x, rq.x);
    let rect_y = interval_interior(rp.y, rq.y);
    let (rminx, rmaxx) = minmax(rp.x, rq.x);
    let (rminy, rmaxy) = minmax(rp.y, rq.y);
    let (lminx, lmaxx) = minmax(line.p.x, line.q.x);
    let (lminy, lmaxy) = minmax(line.p.y, line.q.y);
    if line.p.x == line.q.x {
        rect_x.contains(&line.p.x)
            && (rect_y.contains(&line.p.y)
                || rect_y.contains(&line.q.y)
                || !(lmaxy <= rminy || lminy >= rmaxy))
    } else {
        rect_y.contains(&line.p.y)
            && (rect_x.contains(&line.p.x)
                || rect_x.contains(&line.q.x)
                || !(lmaxx <= rminx || lminx >= rmaxx))
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
        .flat_map(|(ip, p)| {
            points.iter().skip(ip + 1).filter_map(|q| {
                if lines.iter().any(|line| intersects_interior(line, (p, q))) {
                    None
                } else {
                    Some(p.area(q))
                }
            })
        })
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

    let polarity: Vec<bool> = std::iter::once(points[0].polarity(
        points.len() - 1,
        &points[points.len() - 1],
        &points[1],
    ))
    .chain(points.windows(3).enumerate().flat_map(|(i, window)| {
        if let [p, q, r] = window {
            Some(q.polarity(i, p, r))
        } else {
            None
        }
    }))
    .chain(std::iter::once(points[points.len() - 1].polarity(
        points.len() - 2,
        &points[points.len() - 2],
        &points[0],
    )))
    .collect();

    debug_assert!(
        polarity
            .windows(4)
            .all(|corners| !corners[1..].iter().all(|c| *c != corners[0])),
        "Detected knot in input tile loop"
    );
    debug_assert!(
        {
            let counts = polarity.iter().counts();
            counts[&true].abs_diff(counts[&false]) == 4
        },
        "Input tile loop is not closed"
    );

    (solve_a(&points).to_string(), solve_b(&points).to_string())
}
