package day17

import (
	"container/heap"
	"fmt"
	"io"
	"os"
	"strings"
)

func readInput(filename string) [][]int {
	f, err := os.Open(filename)
	if err != nil {
		panic(err)
	}
	defer f.Close()

	data, _ := io.ReadAll(f)
	dataStr := strings.TrimSpace(string(data))
	lines := strings.Split(dataStr, "\n")

	res := make([][]int, len(lines))
	for i, line := range lines {
		res[i] = make([]int, len(line))
		for j, char := range line {
			res[i][j] = int(char) - '0'
		}
	}

	return res
}

type tile struct {
	x, y     int
	dx, dy   int
	distance int
}

type tileHeap []tile

func (t tileHeap) Len() int {
	return len(t)
}

func (t tileHeap) Less(i int, j int) bool {
	return t[i].distance < t[j].distance
}

func (t tileHeap) Swap(i int, j int) {
	t[i], t[j] = t[j], t[i]
}

func (t *tileHeap) Push(x any) {
	*t = append(*t, x.(tile))
}

func (t *tileHeap) Pop() any {
	old := *t
	n := len(old)
	item := old[n-1]
	*t = old[0 : n-1]
	return item
}

type direction struct {
	dx, dy int
}

type visitedKey struct {
	x, y, dx, dy int
}

func solve(input [][]int, minMove, maxMove int) int {
	height := len(input)
	width := len(input[0])

	queue := &tileHeap{
		tile{
			x:        0,
			y:        0,
			dx:       0,
			dy:       0,
			distance: 0,
		},
	}
	heap.Init(queue)

	visited := make(map[visitedKey]bool)

	for queue.Len() > 0 {
		t := heap.Pop(queue).(tile)
		if t.x == width-1 && t.y == height-1 {
			return t.distance
		}

		key := visitedKey{
			x:  t.x,
			y:  t.y,
			dx: t.dx,
			dy: t.dy,
		}
		if visited[key] {
			continue
		}
		visited[key] = true

		var dirs []direction
		if t.dx == 0 && t.dy == 0 {
			dirs = []direction{
				{dx: 1, dy: 0},
				{dx: 0, dy: 1},
			}
		} else {
			dirs = []direction{
				{dx: t.dy, dy: -t.dx},
				{dx: -t.dy, dy: t.dx},
			}
		}

		for _, dir := range dirs {
			x := t.x
			y := t.y
			h := t.distance
			for i := 0; i < maxMove; i++ {
				x += dir.dx
				y += dir.dy

				if x < 0 || x >= width || y < 0 || y >= height {
					break
				}

				h += input[y][x]
				if i+1 >= minMove {
					heap.Push(queue, tile{
						x:        x,
						y:        y,
						dx:       dir.dx,
						dy:       dir.dy,
						distance: h,
					})
				}
			}
		}
	}

	return 0
}

func partOne(input [][]int) int {
	return solve(input, 1, 3)
}

func partTwo(input [][]int) int {
	return solve(input, 4, 10)
}

func Solve() {
	input := readInput("inputs/day_17.txt")

	solutionOne := partOne(input)
	fmt.Println("Part 1:", solutionOne)

	solutionTwo := partTwo(input)
	fmt.Println("Part 2:", solutionTwo)
}
