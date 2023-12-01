package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func check(e error) {
	if e != nil {
		panic(e)
	}
}

func main() {
	file, err := os.Open("input")
	check(err)
	defer file.Close()

	bar := 0
	
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		text := scanner.Text()
		// replace with number in between for numbers like `twone`
		text = strings.ReplaceAll(text, "one", "o1e")
		text = strings.ReplaceAll(text, "two", "t2o")
		text = strings.ReplaceAll(text, "three", "t3e")
		text = strings.ReplaceAll(text, "four", "4")
		text = strings.ReplaceAll(text, "five", "5e")
		text = strings.ReplaceAll(text, "six", "6")
		text = strings.ReplaceAll(text, "seven", "7n")
		text = strings.ReplaceAll(text, "eight", "e8t")
		text = strings.ReplaceAll(text, "nine", "n9e")
		
		i := 0
		x := 0
		y := 0

		for x == 0 {
			s := string(text[i])
			n, err := strconv.Atoi(s)
			if err != nil {
				i += 1
			} else {
				x = n * 10
				i = 0
			}
		}
		for y == 0 {
			s := string(text[len(text) - i - 1])
			n, err := strconv.Atoi(s)
			if err != nil {
				i += 1
			} else {
				y = n
				i = 0
			}
		}

		bar += x + y
	}

	fmt.Println(bar)
}