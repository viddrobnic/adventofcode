package main

import (
	"fmt"
	"os"

	"github.com/viddrobnic/adventofcode/2023/go/day17"
	"github.com/viddrobnic/adventofcode/2023/go/day18"
	"github.com/viddrobnic/adventofcode/2023/go/day19"
	"github.com/viddrobnic/adventofcode/2023/go/day20"
	"github.com/viddrobnic/adventofcode/2023/go/day21"
	"github.com/viddrobnic/adventofcode/2023/go/day22"
	"github.com/viddrobnic/adventofcode/2023/go/day23"
	"github.com/viddrobnic/adventofcode/2023/go/day24"
	"github.com/viddrobnic/adventofcode/2023/go/day25"
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
	case "20":
		day20.Solve()
	case "21":
		day21.Solve()
	case "22":
		day22.Solve()
	case "23":
		day23.Solve()
	case "24":
		day24.Solve()
	case "25":
		day25.Solve()
	default:
		fmt.Println("Day not implemented")
	}
}
