package main

import (
	"fmt"
	"os"
)

func Task1(inp []byte) {
	linelen := 0
	for i := 0; inp[i] != '\n'; i++ {
		linelen++
	}
	gamma := 0
	epsilon := 0
	n := 1
	for i := linelen - 1; i >= 0; i-- {
		if MostCommonInColumn(i, linelen, inp) {
			gamma += n
		} else {
			epsilon += n
		}
		n *= 2
	}
	fmt.Println(gamma * epsilon)
}

func Task2(inp []byte) {
	//fmt.Println(inp)
}

func MostCommonInColumn(col int, linelen int, inp []byte) bool {
	zero := 0
	one := 0
	for i := col; i < len(inp); i += linelen + 1 {
		if inp[i] == '0' {
			zero++
		} else if inp[i] == '1' {
			one++
		} else {
			panic("Unexpected character in input")
		}
	}
	return one > zero
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
