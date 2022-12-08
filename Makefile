all: buildrs buildhs buildc buildzig

buildrs: setup day01rs day02rs day03rs day04rs day05rs

.PHONY: setup
setup:
	mkdir -p target

day01rs: day01/day01.rs
	rustc -o target/$@ $?
day02rs: day02/day02.rs
	rustc -o target/$@ $?
day03rs: day03/day03.rs
	rustc -o target/$@ $?
day04rs: day04/day04.rs
	rustc -o target/$@ $?
day05rs: day05/day05.rs
	rustc -o target/$@ $?
