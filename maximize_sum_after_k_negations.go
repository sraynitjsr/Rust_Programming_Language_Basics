package main

import (
	"fmt"
	"sort"
)

func main() {
	arr := []int{-2, 0, 5, -1, 2}
	k := 4

	sort.Ints(arr)
	i := 0
	for k > 0 && i < len(arr) && arr[i] < 0 {
		arr[i] = -arr[i]
		k--
		i++
	}
	if k%2 != 0 {
		if i == 0 {
			arr[0] = -arr[0]
		} else {
			if i < len(arr) && (arr[i] < -arr[i-1] || i == len(arr)-1) {
				arr[i] = -arr[i]
			} else {
				arr[i-1] = -arr[i-1]
			}
		}
	}
	sum := 0
	for _, num := range arr {
		sum += num
	}

	fmt.Println(sum)
}
