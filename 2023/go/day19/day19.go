package day19

import (
	"fmt"
	"io"
	"os"
	"strconv"
	"strings"
)

type part struct {
	x, m, a, s int
}

type boundPart struct {
	minX, maxX int
	minM, maxM int
	minA, maxA int
	minS, maxS int

	result string
}

type rule struct {
	label     string
	value     int
	operation string

	gotoAfter string
}

type workflow struct {
	rules       []rule
	defaultGoto string
}

func readInput(filename string) (map[string]workflow, []part) {
	f, err := os.Open(filename)
	if err != nil {
		panic(err)
	}
	defer f.Close()

	data, _ := io.ReadAll(f)
	dataStr := strings.TrimSpace(string(data))
	dataParts := strings.Split(dataStr, "\n\n")

	workflowsRaw := strings.Split(dataParts[0], "\n")
	partsRaw := strings.Split(dataParts[1], "\n")

	// Parse workflows
	workflows := make(map[string]workflow)
	for _, w := range workflowsRaw {
		parts := strings.Split(w[0:len(w)-1], "{")
		name := parts[0]

		rulesRaw := strings.Split(parts[1], ",")
		rules := make([]rule, len(rulesRaw)-1)
		for j, r := range rulesRaw[:len(rulesRaw)-1] {
			ruleParts := strings.Split(r, ":")
			value, err := strconv.Atoi(ruleParts[0][2:])
			if err != nil {
				panic(err)
			}

			rules[j] = rule{
				label:     r[0:1],
				value:     value,
				operation: r[1:2],
				gotoAfter: ruleParts[1],
			}
		}

		workflows[name] = workflow{
			rules:       rules,
			defaultGoto: rulesRaw[len(rulesRaw)-1],
		}
	}

	// Parse parts
	parts := make([]part, len(partsRaw))
	for i, p := range partsRaw {
		p = p[1 : len(p)-1]

		categories := strings.Split(p, ",")
		for _, c := range categories {
			prts := strings.Split(c, "=")
			value, err := strconv.Atoi(prts[1])
			if err != nil {
				panic(err)
			}

			switch prts[0] {
			case "x":
				parts[i].x = value
			case "m":
				parts[i].m = value
			case "a":
				parts[i].a = value
			case "s":
				parts[i].s = value
			}
		}
	}

	return workflows, parts
}

func execute(wrkflw workflow, prt part) string {
	for _, r := range wrkflw.rules {
		var value int
		switch r.label {
		case "x":
			value = prt.x
		case "m":
			value = prt.m
		case "a":
			value = prt.a
		case "s":
			value = prt.s
		}

		ok := false
		switch r.operation {
		case "<":
			ok = value < r.value
		case ">":
			ok = value > r.value
		}

		if ok {
			return r.gotoAfter
		}
	}

	return wrkflw.defaultGoto
}

func getBounds(wrkflw workflow, prt boundPart) []boundPart {
	res := make([]boundPart, 0)
	for _, r := range wrkflw.rules {
		newPrt := prt
		if r.label == "x" && r.operation == "<" {
			prt.minX = r.value
			newPrt.maxX = r.value - 1
		}
		if r.label == "x" && r.operation == ">" {
			prt.maxX = r.value
			newPrt.minX = r.value + 1
		}

		if r.label == "m" && r.operation == "<" {
			prt.minM = r.value
			newPrt.maxM = r.value - 1
		}
		if r.label == "m" && r.operation == ">" {
			prt.maxM = r.value
			newPrt.minM = r.value + 1
		}

		if r.label == "a" && r.operation == "<" {
			prt.minA = r.value
			newPrt.maxA = r.value - 1
		}
		if r.label == "a" && r.operation == ">" {
			prt.maxA = r.value
			newPrt.minA = r.value + 1
		}

		if r.label == "s" && r.operation == "<" {
			prt.minS = r.value
			newPrt.maxS = r.value - 1
		}
		if r.label == "s" && r.operation == ">" {
			prt.maxS = r.value
			newPrt.minS = r.value + 1
		}

		newPrt.result = r.gotoAfter
		res = append(res, newPrt)
	}

	prt.result = wrkflw.defaultGoto
	res = append(res, prt)
	return res
}

func partOne(workflows map[string]workflow, parts []part) int {
	result := 0
	for _, p := range parts {
		res := execute(workflows["in"], p)
		for res != "A" && res != "R" {
			res = execute(workflows[res], p)
		}

		if res == "A" {
			result += p.x + p.m + p.a + p.s
		}
	}

	return result
}

func partTwo(workflows map[string]workflow) int {
	result := 0
	parts := []boundPart{
		{
			minX:   1,
			maxX:   4000,
			minM:   1,
			maxM:   4000,
			minA:   1,
			maxA:   4000,
			minS:   1,
			maxS:   4000,
			result: "in",
		},
	}

	for len(parts) > 0 {
		part := parts[0]
		parts = parts[1:]

		if part.minX > part.maxX || part.minM > part.maxM || part.minA > part.maxA || part.minS > part.maxS {
			continue
		}

		if part.result == "R" {
			continue
		}

		if part.result == "A" {
			result += (part.maxX - part.minX + 1) * (part.maxM - part.minM + 1) * (part.maxA - part.minA + 1) * (part.maxS - part.minS + 1)
			continue
		}

		newParts := getBounds(workflows[part.result], part)
		parts = append(parts, newParts...)
	}

	return result
}

func Solve() {
	workflows, parts := readInput("inputs/day_19.txt")
	fmt.Printf("Part One: %d\n", partOne(workflows, parts))
	fmt.Printf("Part Two: %d\n", partTwo(workflows))
}
