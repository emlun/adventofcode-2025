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

use std::collections::HashMap;
use std::collections::HashSet;

use crate::common::Solution;

#[derive(Eq, PartialEq)]
struct Point(i64, i64, i64);
impl Point {
    fn dist2(&self, other: &Self) -> i64 {
        let Self(x1, y1, z1) = self;
        let Self(x2, y2, z2) = other;
        (x1 - x2).pow(2) + (y1 - y2).pow(2) + (z1 - z2).pow(2)
    }
}

fn solve_a(points: &[Point]) -> usize {
    let by_dist: Vec<(usize, usize)> = {
        let by_dist: HashSet<(usize, usize)> = points
            .iter()
            .enumerate()
            .flat_map(|(ip, _)| {
                points
                    .iter()
                    .enumerate()
                    .filter(move |(iq, _)| *iq != ip)
                    .map(move |(iq, _)| (ip, iq))
                    .map(|(ip, iq)| if ip < iq { (ip, iq) } else { (iq, ip) })
            })
            .collect();
        let mut by_dist: Vec<(usize, usize)> = by_dist.into_iter().collect();
        by_dist.sort_by_key(|(ip, iq)| points[*ip].dist2(&points[*iq]));
        by_dist
    };

    let mut circuits: HashMap<usize, HashSet<usize>> = HashMap::new();
    let mut circuit_membership: HashMap<usize, usize> = HashMap::new();
    let mut next_circuit_id = 0;

    for (ip, iq) in by_dist.into_iter().take(1000) {
        match (
            circuit_membership.get(&ip).copied(),
            circuit_membership.get(&iq).copied(),
        ) {
            (None, None) => {
                circuits.insert(next_circuit_id, [ip, iq].into_iter().collect());
                circuit_membership.insert(ip, next_circuit_id);
                circuit_membership.insert(iq, next_circuit_id);
                next_circuit_id += 1;
            }
            (Some(circuit_id), None) => {
                circuits.get_mut(&circuit_id).unwrap().insert(iq);
                circuit_membership.insert(iq, circuit_id);
            }
            (None, Some(circuit_id)) => {
                circuits.get_mut(&circuit_id).unwrap().insert(ip);
                circuit_membership.insert(ip, circuit_id);
            }
            (Some(circuit_id_p), Some(circuit_id_q)) => {
                if circuit_id_p != circuit_id_q {
                    if let Some(circuit_q) = circuits.remove(&circuit_id_q) {
                        let circuit_p = circuits.get_mut(&circuit_id_p).unwrap();
                        for iqq in circuit_q {
                            *circuit_membership.get_mut(&iqq).unwrap() = circuit_id_p;
                            circuit_p.insert(iqq);
                        }
                    }
                }
            }
        }
    }

    let mut lens: Vec<usize> = circuits.values().map(|c| c.len()).collect();
    lens.sort();
    lens.into_iter().rev().take(3).product()
}

pub fn solve(lines: &[String]) -> Solution {
    let boxes: Vec<Point> = lines
        .iter()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (x, rest) = line.split_once(',').unwrap();
            let (y, z) = rest.split_once(',').unwrap();
            Point(x.parse().unwrap(), y.parse().unwrap(), z.parse().unwrap())
        })
        .collect();
    (solve_a(&boxes).to_string(), "".to_string())
}
