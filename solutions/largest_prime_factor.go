package solutions

import (
	"math"
	"fmt"
)

func isPrime(n int) bool {
	sqrt := int(math.Floor(math.Pow(float64(n), 0.5)))
	for i := 2; i < sqrt; i++ {
		if n % i == 0 {
			return false
		}

	}
	return true
}

func LargestPrimeFactor(n float64) float64 {
	largestPrimeFactor := -1.0
	x := 2.0
	for {
		if isPrime(int(n)) {
			return n
		}
		if int(n) % int(x) == 0 {
			fmt.Println("Found prime factor ", x)
			largestPrimeFactor = math.Max(x, largestPrimeFactor)
			n = n / x
			x = 2
		} else {
			x++
		}

	}

	return largestPrimeFactor


}