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

use std::collections::VecDeque;

use crate::common::Solution;

fn neighbors<'a, 'b>(
    rolls: &'a [Vec<bool>],
    (r, c): &'b (usize, usize),
) -> impl Iterator<Item = (usize, usize)> + use<'a, 'b> {
    (-1_isize..=1)
        .flat_map(|dr| (-1_isize..=1).map(move |dc| (dr, dc)))
        .filter(|drc| *drc != (0, 0))
        .filter_map(move |(dr, dc)| {
            let rr = r.checked_add_signed(dr)?;
            let cc = c.checked_add_signed(dc)?;
            Some((rr, cc)).filter(|(rr, cc)| *rr < rolls.len() && *cc < rolls[0].len())
        })
        .filter(|(rr, cc)| rolls[*rr][*cc])
}

fn solve_a(rolls: &[Vec<bool>]) -> usize {
    (0..rolls.len())
        .flat_map(|r| (0..rolls[0].len()).map(move |c| (r, c)))
        .filter(|(r, c)| rolls[*r][*c])
        .filter(|rc| neighbors(&rolls, rc).count() < 4)
        .count()
}

fn solve_b(mut rolls: Vec<Vec<bool>>) -> usize {
    let mut remove_queue: VecDeque<(usize, usize)> = (0..rolls.len())
        .flat_map(|r| (0..rolls[0].len()).map(move |c| (r, c)))
        .filter(|(r, c)| rolls[*r][*c])
        .filter(|rc| neighbors(&rolls, rc).count() < 4)
        .collect::<VecDeque<_>>();
    let mut removed = 0;
    while let Some((r, c)) = remove_queue.pop_front() {
        if rolls[r][c] && neighbors(&rolls, &(r, c)).count() < 4 {
            removed += 1;
            rolls[r][c] = false;
            for rrcc in neighbors(&rolls, &(r, c)) {
                remove_queue.push_back(rrcc);
            }
        }
    }
    removed
}

pub fn solve(lines: &[String]) -> Solution {
    let rolls: Vec<Vec<bool>> = lines
        .iter()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().map(move |ch| ch == '@').collect())
        .collect();
    (solve_a(&rolls).to_string(), solve_b(rolls).to_string())
}
