package day18

import (
	"fmt"
	"io"
	"os"
	"strconv"
	"strings"
)

type instruction struct {
	direction string
	distance  int
	color     string
}

func readInput(filename string) []instruction {
	f, err := os.Open(filename)
	if err != nil {
		panic(err)
	}
	defer f.Close()

	data, _ := io.ReadAll(f)
	dataStr := strings.TrimSpace(string(data))
	lines := strings.Split(dataStr, "\n")

	res := make([]instruction, len(lines))
	for i, line := range lines {
		words := strings.Split(line, " ")

		distance, err := strconv.Atoi(words[1])
		if err != nil {
			panic(err)
		}

		res[i] = instruction{
			direction: words[0],
			distance:  distance,
			color:     strings.ReplaceAll(strings.ReplaceAll(words[2], "(", ""), ")", ""),
		}
	}

	return res
}

func partOne(input []instruction) int {
	currentX, currentY := 0, 0
	area := 0
	boundaryLength := 0

	for _, inst := range input {
		newX, newY := currentX, currentY
		switch inst.direction {
		case "R":
			newX += inst.distance
		case "L":
			newX -= inst.distance
		case "U":
			newY += inst.distance
		case "D":
			newY -= inst.distance
		}

		area += (newY + currentY) * (currentX - newX)
		boundaryLength += inst.distance

		currentX, currentY = newX, newY
	}

	area = area / 2
	if area < 0 {
		area = -area
	}

	intPoints := area + 1 - boundaryLength/2
	return intPoints + boundaryLength
}

func partTwo(input []instruction) int {
	instructions := make([]instruction, len(input))
	for i, inst := range input {
		h := inst.color[1:]
		distance, err := strconv.ParseInt(h[:5], 16, 64)
		if err != nil {
			panic(err)
		}

		dir := ""
		switch h[5] {
		case '0':
			dir = "R"
		case '1':
			dir = "D"
		case '2':
			dir = "L"
		case '3':
			dir = "U"
		}

		instructions[i] = instruction{
			direction: dir,
			distance:  int(distance),
		}
	}

	return partOne(instructions)
}

func Solve() {
	input := readInput("inputs/day_18.txt")

	fmt.Println("Part one:", partOne(input))
	fmt.Println("Part two:", partTwo(input))
}
