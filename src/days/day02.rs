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

use std::collections::HashSet;

use crate::common::Solution;

fn parse_split_n<const N: usize>(s: &str) -> Option<[usize; N]> {
    if s.len() % N == 0 {
        let l = s.len() / N;
        let mut result = [0; N];
        for i in 0..N {
            result[i] = s[i * l..(i + 1) * l].parse().unwrap();
        }
        Some(result)
    } else {
        None
    }
}

fn parse_split(s: &str, n: usize) -> Option<Vec<usize>> {
    if s.len() % n == 0 {
        let l = s.len() / n;
        Some(
            (0..n)
                .map(|i| s[i * l..(i + 1) * l].parse().unwrap())
                .collect(),
        )
    } else {
        None
    }
}

fn solve_a(ranges: &[(&str, &str)]) -> usize {
    ranges
        .iter()
        .flat_map(|(ls, rs)| {
            if ls.len() == rs.len() && ls.len() % 2 != 0 {
                1_usize..=0_usize
            } else {
                let min = if let Some([minl, minr]) = parse_split_n(ls) {
                    if minr > minl {
                        minl + 1
                    } else {
                        minl
                    }
                } else {
                    10_usize.pow(u32::try_from(ls.len().div_ceil(2)).unwrap() - 1)
                };
                let max = if let Some([maxl, maxr]) = parse_split_n(rs) {
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

fn solve_b(ranges: &[(&str, &str)]) -> usize {
    ranges
        .iter()
        .flat_map(|(ls, rs)| {
            (2..=rs.len()).flat_map(move |repeats| {
                if ls.len() == rs.len() && ls.len() % repeats != 0 {
                    1_usize..=0_usize
                } else {
                    let min = if let Some(mins) = parse_split(ls, repeats) {
                        mins.iter().rev().fold(
                            0,
                            |minr, minl| {
                                if minr > *minl {
                                    minl + 1
                                } else {
                                    *minl
                                }
                            },
                        )
                    } else {
                        10_usize.pow(u32::try_from(ls.len().div_ceil(repeats)).unwrap() - 1)
                    };
                    let max = if let Some(maxs) = parse_split(rs, repeats) {
                        maxs.iter().rev().fold(usize::MAX, |maxr, maxl| {
                            if maxr < *maxl {
                                maxl - 1
                            } else {
                                *maxl
                            }
                        })
                    } else {
                        10_usize.pow(u32::try_from(rs.len() / repeats).unwrap()) - 1
                    };

                    min..=max
                }
                .map(move |n| {
                    let log = n.ilog10() + 1;
                    let nn = (0..repeats).fold(0, |nn, i| {
                        nn + 10_usize.pow(log * u32::try_from(i).unwrap()) * n
                    });
                    debug_assert!(nn >= ls.parse().unwrap());
                    debug_assert!(nn <= rs.parse().unwrap());
                    nn
                })
            })
        })
        .collect::<HashSet<usize>>()
        .into_iter()
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

    (solve_a(&ranges).to_string(), solve_b(&ranges).to_string())
}
