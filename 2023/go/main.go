package main

import (
	"fmt"
	"os"

	"github.com/viddrobnic/adventofcode/2023/go/day17"
	"github.com/viddrobnic/adventofcode/2023/go/day18"
	"github.com/viddrobnic/adventofcode/2023/go/day19"
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
	case "18":
		day18.Solve()
	case "19":
		day19.Solve()
	default:
		fmt.Println("Day not implemented")
	}
}
