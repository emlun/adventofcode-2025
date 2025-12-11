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

use std::collections::{HashMap, HashSet, VecDeque};

use crate::common::Solution;

fn solve_a(graph: &HashMap<&str, HashSet<&str>>) -> usize {
    let mut paths = HashMap::new();
    paths.insert("you", 1);
    let mut queue = VecDeque::new();
    queue.push_back("you");
    while let Some(next) = queue.pop_front() {
        if let Some(neighbors) = graph.get(next) {
            for neighbor in neighbors {
                *paths.entry(neighbor).or_insert(0) += 1;
                queue.push_back(neighbor);
            }
        }
    }
    paths["out"]
}

pub fn solve(lines: &[String]) -> Solution {
    let graph: HashMap<&str, HashSet<&str>> = lines
        .iter()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .filter_map(|line| {
            let (key, rest) = line.split_once(':')?;
            let neighbors = rest.trim().split_ascii_whitespace().collect();
            Some((key, neighbors))
        })
        .collect();

    (solve_a(&graph).to_string(), "".to_string())
}
