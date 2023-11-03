package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"reflect"
	"runtime"
	"strings"
	"time"
	"unicode/utf8"
)

func main() {
	run()
}

// https://stackoverflow.com/a/7053871/13123535
func get_function_name(i interface{}) string {
	return runtime.FuncForPC(reflect.ValueOf(i).Pointer()).Name()
}

func run() {
	lines := get_lines("input.txt")
	test_lines := get_lines("test_input.txt")
	benchmark(part1, test_lines)
	benchmark(part2, test_lines)
	benchmark(part1, lines)
	benchmark(part2, lines)
}

func benchmark(f func(InputType) RetType, lines []string) {
	start := time.Now()
	res := f(format(lines))
	name := get_function_name(f)
	fmt.Printf("%s = %v %s %v\n", name, res, strings.Repeat(" ", 30-utf8.RuneCountInString(name)), time.Since(start))
}

// https://stackoverflow.com/a/16615559/13123535
func get_lines(path string) (lines []string) {
	file, err := os.Open(path)
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)
	// optionally, resize scanner's capacity for lines over 64K, see next example
	for scanner.Scan() {
		lines = append(lines, scanner.Text())
	}

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}

	return
}
