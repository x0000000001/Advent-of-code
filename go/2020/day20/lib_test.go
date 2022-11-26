package main

import (
	"fmt"
	"testing"
)

// https://gobyexample.com/testing
func TestLib(t *testing.T) {
	var tests = []struct {
		path            string
		expected_output RetType
		fun             func(InputType) RetType
	}{
		{
			"test_input.txt",
			20899048083289,
			part1,
		},
		{
			"test_input.txt",
			0,
			part2,
		},
	}

	for _, tt := range tests {
		testname := fmt.Sprintf("%s over %s", get_function_name(tt.fun), tt.path)
		t.Run(testname, func(t *testing.T) {
			output := tt.fun(format(get_lines(tt.path)))
			if output != tt.expected_output {
				t.Errorf("got %d, expected %d", output, tt.expected_output)
			}
		})
	}
}
