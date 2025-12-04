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

fn parse_split_middle(s: &str) -> (usize, usize) {
    if s.len() == 1 {
        let si = s.parse().unwrap();
        (si, si)
    } else {
        let l = s.len() / 2;
        let ll = s.len() % 2;
        (s[0..(l + ll)].parse().unwrap(), s[l..].parse().unwrap())
    }
}

fn solve_a(ranges: &[(&str, &str)]) -> usize {
    ranges
        .iter()
        .flat_map(|(ls, rs)| {
            if ls.len() == rs.len() && ls.len() % 2 != 0 {
                1_usize..=0_usize
            } else {
                let min = if ls.len() % 2 == 0 {
                    let (minl, minr) = parse_split_middle(ls);
                    if minr > minl {
                        minl + 1
                    } else {
                        minl
                    }
                } else {
                    10_usize.pow(u32::try_from(ls.len().div_ceil(2)).unwrap() - 1)
                };
                let max = if rs.len() % 2 == 0 {
                    let (maxl, maxr) = parse_split_middle(rs);
                    if maxr < maxl {
                        maxl - 1
                    } else {
                        maxl
                    }
                } else {
                    10_usize.pow(u32::try_from(rs.len() / 2).unwrap()) - 1
                };

                min..=max
            }
            .map(move |n| (ls, rs, n))
        })
        .map(|(ls, rs, n)| {
            let log = n.ilog10() + 1;
            let nn = n + 10_usize.pow(log) * n;
            debug_assert!(nn >= ls.parse().unwrap());
            debug_assert!(nn <= rs.parse().unwrap());
            nn
        })
        .sum()
}

pub fn solve(lines: &[String]) -> Solution {
    let ranges: Vec<(&str, &str)> = lines
        .iter()
        .flat_map(|line| line.split(','))
        .filter(|s| !s.is_empty())
        .map(|s| {
            let (l, r) = s.split_once('-').unwrap();
            (l, r)
        })
        .collect();

    (solve_a(&ranges).to_string(), "".to_string())
}
