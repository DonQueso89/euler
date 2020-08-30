package solutions

func euler(n int) int {
	return (n + 1) * n / 2
}

func MultiplesOfThreeAndFiveBelow(threshold int) int {
	var limitThree, limitFive, limitFifteen int
	if mod := threshold % 3; mod == 0 {
		limitThree = threshold - 3
	} else {
		limitThree = threshold - mod
	}
	if mod := threshold % 5; mod == 0 {
		limitFive = threshold - 5
	} else {
		limitFive = threshold - mod
	}
	if mod := threshold % 15; mod == 0 {
		limitFifteen = threshold - 15
	} else {
		limitFifteen = threshold - mod
	}
	return euler(limitThree / 3) * 3 + euler(limitFive / 5) * 5 - euler(limitFifteen / 15) * 15
}
