#!/bin/sh

# Solutions to Advent of Code 2025
# Copyright (C) 2025  Emil Lundberg <emil@emlun.se>
#
# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.
#
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU General Public License for more details.
#
# You should have received a copy of the GNU General Public License
# along with this program.  If not, see <https://www.gnu.org/licenses/>.

# Exit on error
set -e

YEAR=2025
if [[ -z "$1" ]]; then
  DAY=$(date '+%-d')
  DAY_0=$(date '+%d')
else
  DAY=$(date --date="$YEAR-12-$1" '+%-d')
  DAY_0=$(date --date="$YEAR-12-$1" '+%d')
fi

TRIGGER_TIMESTAMP="${YEAR}-12-${DAY_0}T05:00:01+00:00"
check_time() {
  [[ "$(date -Is -u)" < "${TRIGGER_TIMESTAMP}" ]]
}
if check_time; then
  echo "waiting until $TRIGGER_TIMESTAMP"
  while check_time; do sleep 1; done
fi

curl --silent --cookie "session=${SESSION_COOKIE}" "https://adventofcode.com/${YEAR}/day/${DAY}/input" | tee "inputs/day${DAY_0}.in"

if [[ -a ./inputs/.git ]]; then
  git -C inputs add "day${DAY_0}.in" && git -C inputs commit -m "Add ${YEAR} day ${DAY}"
fi
