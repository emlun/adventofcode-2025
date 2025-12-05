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

use std::ops::RangeInclusive;

use crate::common::Solution;

fn merge<T>(
    a: RangeInclusive<T>,
    b: RangeInclusive<T>,
) -> (RangeInclusive<T>, Option<RangeInclusive<T>>)
where
    T: Copy,
    T: Ord,
{
    if a.start() > b.start() {
        merge(b, a)
    } else if a.start() <= b.start() && a.end() >= b.end() {
        (a, None)
    } else if b.start() > a.end() {
        (a, Some(b))
    } else {
        ((*a.start())..=(*b.end()), None)
    }
}

fn solve_a(fresh: &[RangeInclusive<usize>], ids: &[usize]) -> usize {
    ids.iter()
        .filter(|id| fresh.iter().any(|range| range.contains(id)))
        .count()
}

fn solve_b(mut fresh: Vec<RangeInclusive<usize>>) -> usize {
    fresh.sort_by_key(|range| *range.start());
    let l = fresh.len();
    let mut it = fresh.into_iter();
    let first = it.next().unwrap();
    let (mut merged, last) = it.fold(
        (Vec::with_capacity(l), first),
        |(mut result, last), next| {
            let (a, b) = merge(last, next);
            if let Some(b) = b {
                result.push(a);
                (result, b)
            } else {
                (result, a)
            }
        },
    );
    merged.push(last);
    merged.into_iter().map(|range| range.count()).sum()
}

pub fn solve(lines: &[String]) -> Solution {
    let (fresh, ids, _): (Vec<RangeInclusive<usize>>, Vec<usize>, bool) = lines.iter().fold(
        (Vec::new(), Vec::new(), false),
        |(mut fresh, mut ids, parse_ids), line| {
            if line.is_empty() {
                (fresh, ids, true)
            } else if parse_ids {
                ids.push(line.parse().unwrap());
                (fresh, ids, parse_ids)
            } else {
                let (l, r) = line.trim().split_once('-').unwrap();
                fresh.push(l.parse().unwrap()..=r.parse().unwrap());
                (fresh, ids, parse_ids)
            }
        },
    );
    (
        solve_a(&fresh, &ids).to_string(),
        solve_b(fresh).to_string(),
    )
}
