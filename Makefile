all: buildrs buildhs buildc buildzig

buildrs: setup day01rs

.PHONY: setup
setup:
	mkdir -p target

day01rs: day01/day01.rs
	rustc -o target/$@ $?
