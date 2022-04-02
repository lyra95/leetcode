package p1191

import (
	"testing"
	"github.com/stretchr/testify/assert"
)

func kConcatenationMaxSum(arr []int, k int) int {
	n:= len(arr)
	if n == 0 {
		return 0
	}

	if k <= 1 {
		return max_sub_array_sum(arr)
	}
	
	
	sum := 0
	for _, v := range arr {
		sum += v
	}
	sum = max(sum, 0)
	
	return (sum*(k-2) + max_sub_array_sum(append(arr, arr...)))%1000000007
}

func max(a int, b int) int {
	if a > b {
		return a
	}
	
	return b 
}

func max_sub_array_sum(arr []int) int {
	max := 0
	cur := 0
	for _,v := range arr {
		cur += v
		if max < cur {
			max = cur
		}
		if cur < 0 {
			cur = 0
		}
	}

	return max
}

func TestAll(t *testing.T) {
	assert.Equal(t, 9, kConcatenationMaxSum([]int{1, 2}, 3))
	assert.Equal(t, 2, kConcatenationMaxSum([]int{1, -2, 1}, 5))
	assert.Equal(t, 0, kConcatenationMaxSum([]int{-1, -2}, 7))
}

func TestFoo(t *testing.T) {
	assert.Equal(t, 0, max_sub_array_sum([]int{-1,-2,-1,-2}))
}
