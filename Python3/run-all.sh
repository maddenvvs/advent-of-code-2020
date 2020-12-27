#!/bin/sh

for day in $(seq -w 1 25)
do
    aoc2020 $day "../input/day-$day.input"
done