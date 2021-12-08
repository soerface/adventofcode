package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

func Task1(inp []int) {
	res := 0
	for i := 1; i < len(inp); i++ {
		if inp[i-1] < inp[i] {
			res++
		}
	}
	fmt.Println(res)
}

func Task2(inp []int) {
	Task1(RollingWindow(inp, 3))
}

func RollingWindow(inp []int, windowSize int) []int {
	var out []int
	for i := 0; i < len(inp)-(windowSize-1); i++ {
		n := 0
		for j := 0; j < windowSize; j++ {
			n += inp[i+j]
		}
		out = append(out, n)
	}
	return out
}

func InputToIntList(inp []byte) []int {
	var out []int
	var str strings.Builder
	j := 0
	for i := 0; i < len(inp); i++ {
		c := inp[i]
		if c != '\n' {
			str.WriteByte(c)
		}
		if c == 10 || i == len(inp)-1 {
			n, err := strconv.Atoi(str.String())
			if err != nil {
				panic(err)
			}
			out = append(out, n)
			j++
			str.Reset()
			continue
		}
	}
	return out
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
	Task1(InputToIntList(data))
	Task2(InputToIntList(data))
}
