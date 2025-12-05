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

fn merge<T>(a: &RangeInclusive<T>, b: &RangeInclusive<T>) -> Option<RangeInclusive<T>>
where
    T: Copy,
    T: Ord,
{
    if a.start() > b.start() {
        merge(b, a)
    } else if b.start() > a.end() {
        None
    } else {
        Some((*a.start())..=(*std::cmp::max(a.end(), b.end())))
    }
}

fn merge_all<T>(mut ranges: Vec<RangeInclusive<T>>) -> Vec<RangeInclusive<T>>
where
    T: Copy,
    T: Ord,
{
    let mut i_current = 0;
    for ii in 1..ranges.len() {
        if let Some(merged) = merge(&ranges[i_current], &ranges[ii]) {
            ranges[i_current] = merged;
        } else {
            i_current += 1;
            ranges[i_current] = ranges[ii].clone();
        }
    }
    ranges.truncate(i_current + 1);
    ranges
}

fn solve_a(fresh: &[RangeInclusive<usize>], ids: &[usize]) -> usize {
    ids.iter()
        .filter(|id| {
            let max = fresh.partition_point(|range| range.start() < id);
            fresh[..max].iter().rev().any(|range| range.contains(id))
        })
        .count()
}

fn solve_b(fresh: Vec<RangeInclusive<usize>>) -> usize {
    fresh.into_iter().map(|range| range.count()).sum()
}

pub fn solve(lines: &[String]) -> Solution {
    let (fresh, ids): (Vec<RangeInclusive<usize>>, Vec<usize>) = {
        let mut fresh = Vec::new();
        let mut it = lines.iter();
        while let Some((l, r)) = it.next().and_then(|line| line.trim().split_once('-')) {
            fresh.push(l.parse().unwrap()..=r.parse().unwrap());
        }
        fresh.sort_by_key(|range| *range.start());
        fresh = merge_all(fresh);
        let ids = it.flat_map(|line| line.trim().parse().ok()).collect();
        (fresh, ids)
    };
    (
        solve_a(&fresh, &ids).to_string(),
        solve_b(fresh).to_string(),
    )
}
