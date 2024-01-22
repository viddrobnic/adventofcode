package day21

import (
	"fmt"
	"io"
	"os"
	"strings"
)

type coordinate struct {
	x, y int
}

func readInput(filename string) []string {
	f, err := os.Open(filename)
	if err != nil {
		panic(err)
	}
	defer f.Close()

	data, _ := io.ReadAll(f)
	dataStr := strings.TrimSpace(string(data))
	return strings.Split(dataStr, "\n")
}

func findStart(grid []string) coordinate {
	for y, line := range grid {
		for x, c := range line {
			if c == 'S' {
				return coordinate{
					x: x,
					y: y,
				}
			}
		}
	}

	panic("No start found")
}

var dirs = []coordinate{
	{
		x: -1,
		y: 0,
	},
	{
		x: 1,
		y: 0,
	},
	{
		x: 0,
		y: -1,
	},
	{
		x: 0,
		y: 1,
	},
}

func walkFor(grid []string, lengths []int, wraps bool) []int {
	height := len(grid)
	width := len(grid[0])

	start := findStart(grid)

	res := make([]int, 0, len(lengths))
	visited := map[coordinate]bool{
		start: true,
	}

	maxSteps := lengths[len(lengths)-1]
	next := lengths[0]
	lengths = lengths[1:]
	for i := 1; i <= maxSteps; i++ {
		newVisited := make(map[coordinate]bool)
		for coord := range visited {
			for _, dir := range dirs {
				x := coord.x + dir.x
				y := coord.y + dir.y

				if !wraps && (x < 0 || x >= width || y < 0 || y >= height) {
					continue
				}

				cx := (x%width + width) % width
				cy := (y%height + height) % height
				if grid[cy][cx] != '#' {
					newVisited[coordinate{
						x: x,
						y: y,
					}] = true
				}
			}
		}

		visited = newVisited

		if i == next {
			res = append(res, len(visited))
			if len(lengths) > 0 {
				next = lengths[0]
				lengths = lengths[1:]
			}
		}
	}

	return res
}

func partOne(grid []string) int {
	res := walkFor(grid, []int{64}, false)
	return res[0]
}

func partTwo(grid []string) int {
	start := findStart(grid)

	size := len(grid)
	st := start.x

	res := walkFor(grid, []int{st, st + size, st + 2*size}, true)

	a := (res[2] - 2*res[1] + res[0]) / 2
	b := (res[1] - res[0]) - a
	c := res[0]

	x := 26501365 / size
	return a*x*x + b*x + c
}

func Solve() {
	input := readInput("inputs/day_21.txt")

	resOne := partOne(input)
	fmt.Printf("Part One: %d\n", resOne)

	resTwo := partTwo(input)
	fmt.Printf("Part Two: %d\n", resTwo)
}
