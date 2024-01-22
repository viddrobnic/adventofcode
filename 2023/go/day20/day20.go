package day20

import (
	"fmt"
	"io"
	"os"
	"strings"
)

const broadcasterName = "broadcaster"

type Pulse int

const (
	PulseLow Pulse = iota
	PulseHigh
)

type Modules map[string]Module

type Module interface {
	SendPulse(from string, pulse Pulse) ([]string, Pulse)
	GetReceivers() []string
	Reset()
}

type ModuleBroadcast struct {
	receivers []string
}

func (mb *ModuleBroadcast) SendPulse(_ string, pulse Pulse) ([]string, Pulse) {
	return mb.receivers, pulse
}

func (mb *ModuleBroadcast) GetReceivers() []string {
	return mb.receivers
}

func (mb *ModuleBroadcast) Reset() {
}

type ModuleFlipFlop struct {
	name      string
	state     Pulse
	receivers []string
}

func (mf *ModuleFlipFlop) SendPulse(_ string, pulse Pulse) ([]string, Pulse) {
	if pulse == PulseHigh {
		return []string{}, PulseLow
	}

	if mf.state == PulseLow {
		mf.state = PulseHigh
	} else {
		mf.state = PulseLow
	}

	return mf.receivers, mf.state
}

func (mf *ModuleFlipFlop) GetReceivers() []string {
	return mf.receivers
}

func (mf *ModuleFlipFlop) Reset() {
	mf.state = PulseLow
}

type ModuleConjunction struct {
	name      string
	state     map[string]Pulse
	receivers []string
}

func (mc *ModuleConjunction) SendPulse(from string, pulse Pulse) ([]string, Pulse) {
	mc.state[from] = pulse

	allHigh := true
	for _, s := range mc.state {
		if s == PulseLow {
			allHigh = false
			break
		}
	}

	var pulseToSend Pulse
	if allHigh {
		pulseToSend = PulseLow
	} else {
		pulseToSend = PulseHigh
	}

	return mc.receivers, pulseToSend
}

func (mc *ModuleConjunction) GetReceivers() []string {
	return mc.receivers
}

func (mc *ModuleConjunction) Reset() {
	for k := range mc.state {
		mc.state[k] = PulseLow
	}
}

func moduleFromLine(line string) (string, Module) {
	parts := strings.Split(line, "->")

	modulePart := strings.TrimSpace(parts[0])
	receivers := strings.Split(strings.TrimSpace(parts[1]), ", ")
	if modulePart == broadcasterName {
		return broadcasterName, &ModuleBroadcast{
			receivers: receivers,
		}

	}

	moduleType := modulePart[0]
	moduleName := modulePart[1:]

	switch moduleType {
	case '%':
		return moduleName, &ModuleFlipFlop{
			name:      moduleName,
			state:     PulseLow,
			receivers: receivers,
		}
	case '&':
		return moduleName, &ModuleConjunction{
			name:      moduleName,
			state:     map[string]Pulse{},
			receivers: receivers,
		}
	default:
		panic("Unknown module type: " + string(moduleType))
	}
}

func readInput(filename string) map[string]Module {
	f, err := os.Open(filename)
	if err != nil {
		panic(err)
	}
	defer f.Close()

	data, _ := io.ReadAll(f)
	dataStr := strings.TrimSpace(string(data))

	res := make(map[string]Module)
	for _, line := range strings.Split(dataStr, "\n") {
		name, module := moduleFromLine(line)
		res[name] = module
	}

	// Populate conjunction modules inputs
	for name, module := range res {
		for _, receiver := range module.GetReceivers() {
			rm, ok := res[receiver].(*ModuleConjunction)
			if !ok {
				continue
			}

			rm.state[name] = PulseLow
		}
	}

	return res
}

type node struct {
	name  string
	from  string
	pulse Pulse
}

func handleButtonPress(input map[string]Module, countingFor string) (lowCount, highCount int, pulses []string) {
	q := []node{
		{
			name:  broadcasterName,
			pulse: PulseLow,
		},
	}

	lowCount = 1
	highCount = 0
	pulses = []string{}

	for len(q) > 0 {
		n := q[0]
		q = q[1:]

		module, ok := input[n.name]
		if !ok {
			continue
		}

		toSend, pulse := module.SendPulse(n.from, n.pulse)
		for _, receiver := range toSend {
			if receiver == countingFor && pulse == PulseHigh {
				pulses = append(pulses, n.name)
			}

			q = append(q, node{
				name:  receiver,
				from:  n.name,
				pulse: pulse,
			})
		}

		if pulse == PulseLow {
			lowCount += len(toSend)
		} else {
			highCount += len(toSend)
		}
	}

	return lowCount, highCount, pulses
}

func partOne(input map[string]Module) int {
	lowCount, highCount := 0, 0
	for i := 0; i < 1000; i++ {
		l, h, _ := handleButtonPress(input, "")
		lowCount += l
		highCount += h
	}

	return lowCount * highCount
}

func partTwo(input map[string]Module) uint64 {
	predecessor := ""
	nrToGet := 0

outer:
	for name, module := range input {
		for _, rec := range module.GetReceivers() {
			if rec == "rx" {
				predecessor = name
				pm := input[predecessor].(*ModuleConjunction)
				nrToGet = len(pm.state)

				break outer
			}
		}
	}

	pulsesCount := map[string]uint64{}
	for i := uint64(1); true; i++ {
		_, _, pulses := handleButtonPress(input, predecessor)
		for _, p := range pulses {
			if _, ok := pulsesCount[p]; !ok {
				pulsesCount[p] = i
			}
		}

		if len(pulsesCount) == nrToGet {
			break
		}
	}

	res := uint64(1)
	for _, v := range pulsesCount {
		res *= v
	}
	return res

}

func Solve() {
	input := readInput("inputs/day_20.txt")
	resOne := partOne(input)
	fmt.Printf("Part One: %d\n", resOne)

	for _, module := range input {
		module.Reset()
	}
	resTwo := partTwo(input)
	fmt.Printf("Part Two: %d\n", resTwo)
}
