package day22

import (
	"fmt"
	"io"
	"os"
	"sort"
	"strconv"
	"strings"
)

type point struct {
	x, y, z int
}

type cube struct {
	start, end point
}

func toIntSlice(str string) []int {
	parts := strings.Split(str, ",")
	nums := make([]int, len(parts))
	for i, p := range parts {
		var err error
		nums[i], err = strconv.Atoi(p)
		if err != nil {
			panic(err)
		}
	}

	return nums
}

func order(a, b point) (point, point) {
	if a.x > b.x {
		return b, a
	}

	if a.y > b.y {
		return b, a
	}

	if a.z > b.z {
		return b, a
	}

	return a, b
}

func readInput(filename string) []cube {
	f, err := os.Open(filename)
	if err != nil {
		panic(err)
	}
	defer f.Close()

	data, _ := io.ReadAll(f)
	dataStr := strings.TrimSpace(string(data))

	cubesRaw := strings.Split(dataStr, "\n")
	cubes := make([]cube, len(cubesRaw))
	for i, c := range cubesRaw {
		parts := strings.Split(c, "~")
		one := toIntSlice(parts[0])
		two := toIntSlice(parts[1])

		p1 := point{
			x: one[0],
			y: one[1],
			z: one[2],
		}
		p2 := point{
			x: two[0],
			y: two[1],
			z: two[2],
		}

		cubes[i].start, cubes[i].end = order(p1, p2)
	}

	return cubes
}

func calculateRelations(cubes []cube) (supports, supportedBy map[int]map[int]bool) {
	sort.Slice(cubes, func(i, j int) bool {
		return cubes[i].start.z < cubes[j].start.z
	})

	gridSize := 400
	grid := make([][][]int, gridSize)
	for z := 0; z < gridSize; z++ {
		grid[z] = make([][]int, gridSize)
		for y := 0; y < gridSize; y++ {
			grid[z][y] = make([]int, gridSize)
			for x := 0; x < gridSize; x++ {
				grid[z][y][x] = -1
			}
		}
	}

	for idx, cube := range cubes {
		zOffset := 1
	outer:
		for cube.start.z-zOffset > 0 {
			for x := cube.start.x; x <= cube.end.x; x++ {
				for y := cube.start.y; y <= cube.end.y; y++ {
					if grid[cube.start.z-zOffset][y][x] != -1 {
						break outer
					}
				}
			}

			zOffset++
		}

		zOffset--

		for x := cube.start.x; x <= cube.end.x; x++ {
			for y := cube.start.y; y <= cube.end.y; y++ {
				for z := cube.start.z; z <= cube.end.z; z++ {
					grid[z-zOffset][y][x] = idx
				}
			}
		}
	}

	supports = make(map[int]map[int]bool)
	supportedBy = make(map[int]map[int]bool)

	for idx := range cubes {
		supports[idx] = make(map[int]bool)
		supportedBy[idx] = make(map[int]bool)
	}

	for z := 1; z < gridSize; z++ {
		for y := 0; y < gridSize; y++ {
			for x := 0; x < gridSize; x++ {
				current := grid[z][y][x]
				if current == -1 {
					continue
				}

				above := grid[z+1][y][x]
				if above != -1 && above != current {
					supports[current][above] = true
					supportedBy[above][current] = true
				}
			}
		}
	}

	return supports, supportedBy
}

func partOne(supports, supportedBy map[int]map[int]bool) int {
	res := 0
	for _, supCubes := range supports {
		canRemove := true
		for subCube := range supCubes {
			if len(supportedBy[subCube]) == 1 {
				canRemove = false
				break
			}
		}

		if canRemove {
			res++
		}
	}

	return res
}

func partTwo(supports, supportedBy map[int]map[int]bool) int {
	nrCubes := len(supports)

	res := 0
	for cube := 0; cube < nrCubes; cube++ {
		removedSet := map[int]bool{cube: true}
		prevSize := 0
		for len(removedSet) != prevSize {
			prevSize = len(removedSet)

			for removedCube := range removedSet {
				for supCube := range supports[removedCube] {
					supBy := supportedBy[supCube]
					isSupported := true
					for supByCube := range supBy {
						if !removedSet[supByCube] {
							isSupported = false
							break
						}
					}

					if isSupported {
						removedSet[supCube] = true
					}
				}
			}
		}

		res += len(removedSet) - 1
	}

	return res
}

func Solve() {
	input := readInput("inputs/day_22.txt")
	supports, supportedBy := calculateRelations(input)

	solutionOne := partOne(supports, supportedBy)
	fmt.Printf("Part one: %d\n", solutionOne)

	solutionTwo := partTwo(supports, supportedBy)
	fmt.Printf("Part two: %d\n", solutionTwo)
}
