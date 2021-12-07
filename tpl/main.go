package main

import (
	"fmt"
	"os"
)

func Task1(inp []byte) {
	fmt.Println(inp)
}

func Task2(inp []byte) {
	fmt.Println(inp)
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
