package main

import (
	"fmt"
	"os"

	"github.com/viddrobnic/adventofcode/2023/go/day17"
)

func main() {
	if len(os.Args) != 2 {
		fmt.Println("Usage: aoc <day>")
		return
	}

	day := os.Args[1]
	switch day {
	case "17":
		day17.Solve()
	default:
		fmt.Println("Day not implemented")
	}
}
