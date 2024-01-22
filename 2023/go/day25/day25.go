package day25

import (
	"fmt"
	"io"
	"math"
	"math/rand"
	"os"
	"strings"
)

type graph struct {
	edges map[string][]string
	nodes []string
}

func (g *graph) addConnection(node, connection string) {
	if _, ok := g.edges[node]; !ok {
		g.nodes = append(g.nodes, node)
	}

	g.edges[node] = append(g.edges[node], connection)
}

func (g graph) contract(n1, n2 string) graph {
	node := fmt.Sprintf("%s_%s", n1, n2)
	neighs1 := g.edges[n1]
	neighs2 := g.edges[n2]

	// Add the joined node
	newG := graph{
		edges: make(map[string][]string),
		nodes: []string{},
	}
	for _, neigh := range neighs1 {
		if neigh == n1 || neigh == n2 {
			continue
		}
		newG.addConnection(node, neigh)
		// newG.addConnection(neigh, node)
	}
	for _, neigh := range neighs2 {
		if neigh == n1 || neigh == n2 {
			continue
		}
		newG.addConnection(node, neigh)
		// newG.addConnection(neigh, node)
	}

	// Copy all other nodes
	for n, neighs := range g.edges {
		if n == n1 || n == n2 {
			continue
		}

		for _, neigh := range neighs {
			if neigh == n1 || neigh == n2 {
				newG.addConnection(n, node)
			} else {
				newG.addConnection(n, neigh)
			}
		}
	}

	return newG
}

func readInput(filename string) graph {
	f, err := os.Open(filename)
	if err != nil {
		panic(err)
	}
	defer f.Close()

	data, _ := io.ReadAll(f)
	dataStr := strings.TrimSpace(string(data))
	rows := strings.Split(dataStr, "\n")

	g := graph{
		edges: make(map[string][]string),
		nodes: []string{},
	}
	for _, row := range rows {
		parts := strings.Split(row, ": ")
		node := parts[0]
		connections := strings.Split(parts[1], " ")

		for _, connection := range connections {
			g.addConnection(node, connection)
			g.addConnection(connection, node)
		}
	}

	return g
}

func karger(g graph) graph {
	for len(g.nodes) > 2 {
		n1 := g.nodes[rand.Intn(len(g.nodes))]
		neigs := g.edges[n1]
		n2 := neigs[rand.Intn(len(neigs))]

		g = g.contract(n1, n2)
	}

	return g
}

func partOne(g graph) int {
	n := float64(len(g.nodes))
	repeat := int(n * n * math.Log(n))

	var partition1, partition2 string
	for i := 0; i < repeat; i++ {
		newG := karger(g)
		edges1 := newG.edges[newG.nodes[0]]
		edges2 := newG.edges[newG.nodes[1]]

		len1 := len(edges1)
		len2 := len(edges2)
		if len1 != len2 {
			panic("different lengths of edges")
		}

		if len1 == 3 {
			partition1 = newG.nodes[0]
			partition2 = newG.nodes[1]
			break
		}
	}

	size1 := strings.Count(partition1, "_") + 1
	size2 := strings.Count(partition2, "_") + 1

	return size1 * size2
}

func Solve() {
	input := readInput("inputs/day_25.txt")
	solutionOne := partOne(input)
	fmt.Printf("Part one: %d\n", solutionOne)

}
