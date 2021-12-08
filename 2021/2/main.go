package main

import (
	"fmt"
	"os"
	"strconv"
)

func Task1(inp []byte) {
	pos := 0
	depth := 0
	s := 0
	var cmd string
	for i := 0; i < len(inp); i++ {
		c := inp[i]
		if c == ' ' {
			cmd = string(inp[s:i])
			s = i + 1
		} else if c == '\n' {
			v, err := strconv.Atoi(string(inp[s:i]))
			if err != nil {
				panic(err)
			}
			switch cmd {
			case "forward":
				pos += v
			case "down":
				depth += v
			case "up":
				depth -= v
			}
			s = i + 1
		}
	}
	fmt.Println(pos, depth, pos*depth)
}

func Task2(inp []byte) {
	pos := 0
	depth := 0
	aim := 0
	s := 0
	var cmd string
	for i := 0; i < len(inp); i++ {
		c := inp[i]
		if c == ' ' {
			cmd = string(inp[s:i])
			s = i + 1
		} else if c == '\n' {
			v, err := strconv.Atoi(string(inp[s:i]))
			if err != nil {
				panic(err)
			}
			switch cmd {
			case "forward":
				pos += v
				depth += aim * v
			case "down":
				aim += v
			case "up":
				aim -= v
			}
			s = i + 1
		}
	}
	fmt.Println(pos, depth, pos*depth)
}

func main() {
	if len(os.Args) < 2 {
		fmt.Println("Path of input file expected as argument")
		os.Exit(1)
	}
	data, err := os.ReadFile(os.Args[1])
	if err != nil {
		panic(err)
	}
	Task1(data)
	Task2(data)
}
