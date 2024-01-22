package day23

import (
	"fmt"
	"io"
	"os"
	"strings"
)

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

var diffs = [][]int{
	{-1, 0},
	{1, 0},
	{0, -1},
	{0, 1},
}

func dfs(grid []string, pathset [][]bool, pathLen int, x, y, endX, endY int, oneWay bool) int {
	height := len(grid)
	width := len(grid[0])

	if x == endX && y == endY {
		return pathLen
	}

	pathset[y][x] = true
	defer func() {
		pathset[y][x] = false
	}()

	neighbors := diffs
	if oneWay {
		switch grid[y][x] {
		case '>':
			neighbors = [][]int{{1, 0}}
		case '<':
			neighbors = [][]int{{-1, 0}}
		case '^':
			neighbors = [][]int{{0, -1}}
		case 'v':
			neighbors = [][]int{{0, 1}}
		}
	}

	best := 0
	for _, diff := range neighbors {
		nx := x + diff[0]
		ny := y + diff[1]
		if nx < 0 || nx >= width || ny < 0 || ny >= height {
			continue
		}
		if grid[ny][nx] == '#' {
			continue
		}
		if pathset[ny][nx] {
			continue
		}

		l := dfs(grid, pathset, pathLen+1, nx, ny, endX, endY, oneWay)
		best = max(best, l)
	}

	return best
}

func solve(grid []string, oneWay bool) int {
	height := len(grid)
	width := len(grid[0])

	// Find start
	startX := 0
	for x, c := range grid[0] {
		if c == '.' {
			startX = x
			break
		}
	}

	// Find end
	endX := 0
	for x, c := range grid[height-1] {
		if c == '.' {
			endX = x
			break
		}
	}

	pathset := make([][]bool, height)
	for y := range pathset {
		pathset[y] = make([]bool, width)
	}

	return dfs(grid, pathset, 0, startX, 0, endX, height-1, oneWay)
}

func partOne(grid []string) int {
	return solve(grid, true)
}

func partTwo(grid []string) int {
	return solve(grid, false)
}

func Solve() {
	input := readInput("inputs/day_23.txt")
	solutionOne := partOne(input)
	fmt.Printf("Part One: %d\n", solutionOne)

	solutionTwo := partTwo(input)
	fmt.Printf("Part Two: %d\n", solutionTwo)

}
