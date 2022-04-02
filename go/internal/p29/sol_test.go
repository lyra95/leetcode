package p29

import (
	"github.com/stretchr/testify/assert"
	"testing"
)

const max int = 1 << 31

func divide(dividend int, divisor int) int {
	if dividend == -max && divisor == -1 {
		return max - 1
	}

	if dividend < 0 {
		return divide(-dividend, -divisor)
	}

	if divisor < 0 {
		return -divide(dividend, -divisor)
	}

	quotient := 0
	for dividend >= divisor {
		count := 1
		cur := divisor
		for dividend >= cur<<1 {
			cur = cur << 1
			count = count << 1
		}
		dividend -= cur
		quotient += count
	}
	return quotient
}

func TestPositive(t *testing.T) {
	assert.Equal(t, divide(10, 3), 3)
}

func TestNegative(t *testing.T) {
	assert.Equal(t, divide(-10, -3), 3)
	assert.Equal(t, divide(-10, 3), -3)
	assert.Equal(t, divide(10, -3), -3)
}
