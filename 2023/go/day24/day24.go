package day24

import (
	"fmt"
	"io"
	"math"
	"os"
	"strconv"
	"strings"
)

type vec3 struct {
	x, y, z float64
}

type hailstone struct {
	position, velocity vec3
}

func (h hailstone) pointAt(t float64) vec3 {
	return vec3{
		x: h.position.x + h.velocity.x*t,
		y: h.position.y + h.velocity.y*t,
		z: h.position.z + h.velocity.z*t,
	}
}

func (v vec3) add(v2 vec3) vec3 {
	return vec3{
		x: v.x + v2.x,
		y: v.y + v2.y,
		z: v.z + v2.z,
	}
}

func (v vec3) sub(v2 vec3) vec3 {
	return vec3{
		x: v.x - v2.x,
		y: v.y - v2.y,
		z: v.z - v2.z,
	}
}

func (v vec3) mul(s float64) vec3 {
	return vec3{
		x: v.x * s,
		y: v.y * s,
		z: v.z * s,
	}
}

func (v vec3) dot(v2 vec3) float64 {
	return v.x*v2.x + v.y*v2.y + v.z*v2.z
}

func (v vec3) cross(v2 vec3) vec3 {
	return vec3{
		x: v.y*v2.z - v.z*v2.y,
		y: v.z*v2.x - v.x*v2.z,
		z: v.x*v2.y - v.y*v2.x,
	}
}

func strToVec3(str string) vec3 {
	parts := strings.Split(strings.TrimSpace(str), ",")
	x, _ := strconv.ParseFloat(strings.TrimSpace(parts[0]), 64)
	y, _ := strconv.ParseFloat(strings.TrimSpace(parts[1]), 64)
	z, _ := strconv.ParseFloat(strings.TrimSpace(parts[2]), 64)
	return vec3{x, y, z}
}

func readInput(filename string) []hailstone {
	f, err := os.Open(filename)
	if err != nil {
		panic(err)
	}
	defer f.Close()

	data, _ := io.ReadAll(f)
	dataStr := strings.TrimSpace(string(data))

	lines := strings.Split(dataStr, "\n")
	res := make([]hailstone, len(lines))
	for i, line := range lines {
		parts := strings.Split(line, " @ ")
		res[i] = hailstone{
			position: strToVec3(parts[0]),
			velocity: strToVec3(parts[1]),
		}
	}

	return res
}

func intersection2d(h1, h2 hailstone) (ok bool, t1, t2 float64) {
	a0 := h1.position.x
	a := h1.velocity.x
	b0 := h1.position.y
	b := h1.velocity.y
	c0 := h2.position.x
	c := h2.velocity.x
	d0 := h2.position.y
	d := h2.velocity.y

	if b*c == a*d {
		return false, 0, 0
	}

	t2 = (d0 + (b*a0)/a - b0 - (b*c0)/a) / ((b*c)/a - d)
	t1 = (c0 + t2*c - a0) / a
	return true, t1, t2
}

func intersection3dAux(h1, h2 hailstone) (ok bool, t1, t2 float64) {
	ok, t1, t2 = intersection2d(h1, h2)
	if !ok {
		return false, 0, 0
	}

	z1 := h1.position.z + h1.velocity.z*t1
	z2 := h2.position.z + h2.velocity.z*t2
	if z1 != z2 {
		return false, 0, 0
	}

	return true, t1, t2

}

// intersection3d calculates intersection of two lines in 3D space.
// It works by trying to calculate a solution for 2D intersection,
// since this method is already implemented. If the solution is not
// found, the coordinates are rotated and the method is called again.
// This way we try all 3 possible projections to xy, yz and xz planes.
func intersection3d(h1, h2 hailstone) (ok bool, t1, t2 float64) {
	ok, t1, t2 = intersection3dAux(h1, h2)
	if ok {
		return true, t1, t2
	}

	h1.position = vec3{h1.position.y, h1.position.z, h1.position.x}
	h1.velocity = vec3{h1.velocity.y, h1.velocity.z, h1.velocity.x}
	h2.position = vec3{h2.position.y, h2.position.z, h2.position.x}
	h2.velocity = vec3{h2.velocity.y, h2.velocity.z, h2.velocity.x}

	ok, t1, t2 = intersection3dAux(h1, h2)
	if ok {
		return true, t1, t2
	}

	h1.position = vec3{h1.position.y, h1.position.z, h1.position.x}
	h1.velocity = vec3{h1.velocity.y, h1.velocity.z, h1.velocity.x}
	h2.position = vec3{h2.position.y, h2.position.z, h2.position.x}
	h2.velocity = vec3{h2.velocity.y, h2.velocity.z, h2.velocity.x}

	ok, t1, t2 = intersection3dAux(h1, h2)
	if ok {
		return true, t1, t2
	}

	return false, 0, 0
}

func partOne(input []hailstone) int {
	minCoord := 200000000000000.0
	maxCoord := 400000000000000.0

	res := 0
	for i := 0; i < len(input)-1; i++ {
		for j := i + 1; j < len(input); j++ {
			h1 := input[i]
			h2 := input[j]

			ok, t1, t2 := intersection2d(h1, h2)
			if ok && t1 >= 0 && t2 >= 0 {
				intersection := h1.pointAt(t1)
				x := intersection.x
				y := intersection.y

				if x >= minCoord && x <= maxCoord && y >= minCoord && y <= maxCoord {
					res++
				}
			}
		}
	}

	return res
}
func partTwo(input []hailstone) int {
	// Find three independent velocity vectors
	h1 := input[0]
	var h2, h3 hailstone
	for i := 1; i < len(input); i++ {
		h2 = input[i]
		c := h1.velocity.cross(h2.velocity)
		if c.x != 0 || c.y != 0 || c.z != 0 {
			break
		}
	}

	for i := 2; i < len(input); i++ {
		h3 = input[i]
		c1 := h1.velocity.cross(h3.velocity)
		c2 := h2.velocity.cross(h3.velocity)
		if (c1.x != 0 || c1.y != 0 || c1.z != 0) && (c2.x != 0 || c2.y != 0 || c2.z != 0) {
			break
		}
	}

	// Calculate vectors and constants for 3 plane equations.
	a := h1.position.sub(h2.position).cross(h1.velocity.sub(h2.velocity))
	A := h1.position.sub(h2.position).dot(h1.velocity.cross(h2.velocity))
	b := h1.position.sub(h3.position).cross(h1.velocity.sub(h3.velocity))
	B := h1.position.sub(h3.position).dot(h1.velocity.cross(h3.velocity))
	c := h2.position.sub(h3.position).cross(h2.velocity.sub(h3.velocity))
	C := h2.position.sub(h3.position).dot(h2.velocity.cross(h3.velocity))

	// Base vector for rock velocity: vr = p * b1 + q * b2 + r * b3
	b1 := b.cross(c)
	b2 := c.cross(a)
	b3 := a.cross(b)

	// Find integer solution for p, q, r
	p := A / b1.dot(a)
	q := B / b2.dot(b)
	r := C / b3.dot(c)

	// Calculate rock velocity and round to integer coords
	vr := b1.mul(p).add(b2.mul(q)).add(b3.mul(r))
	vr.x = math.Round(vr.x)
	vr.y = math.Round(vr.y)
	vr.z = math.Round(vr.z)

	// Find intersection of two changed lines
	l1 := hailstone{
		position: h1.position,
		velocity: h1.velocity.sub(vr),
	}
	l2 := hailstone{
		position: h2.position,
		velocity: h2.velocity.sub(vr),
	}

	ok, t1, _ := intersection3d(l1, l2)
	if !ok {
		panic("No intersection found")
	}

	// Calculate position of intersection
	x := l1.position.x + l1.velocity.x*t1
	y := l1.position.y + l1.velocity.y*t1
	z := l1.position.z + l1.velocity.z*t1

	return int(x + y + z)
}

func Solve() {
	input := readInput("inputs/day_24.txt")
	solutionOne := partOne(input)
	fmt.Printf("Part 1: %d\n", solutionOne)

	solutionTwo := partTwo(input)
	fmt.Printf("Part 2: %d\n", solutionTwo)
}
