package solutions

func SumEvenFibonaccis(limit int) int {
	r := 0
	n := 2
	last := 1
	for n < limit {
		if n % 2 == 0 {
			r += n
		}
		n += last
		last = n - last
	}
	return r
}