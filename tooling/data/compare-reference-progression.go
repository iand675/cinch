// Public comparison pipelines driven by scalar operand phases without remainder allocation.
package main

import (
	"cmp"
	"fmt"
	"math"
	"os"
	"strconv"
	"time"
)

// main times runtime operand progression and independently observes the public comparison APIs.
func main() {
	n, _ := strconv.Atoi(os.Args[2])
	mode := os.Args[1]
	checksum := int64(0)
	start := time.Now()
	x, y := int64(-128), int64(-96)
	phase17, phase5, phase3 := 0, 0, 0
	for i := 0; i < n; i++ {
		switch mode {
		case "integer":
			checksum += int64(cmp.Compare(x, y))
			if cmp.Less(x, y) {
				checksum += 3
			}
		case "float":
			a := float64(x)
			b := float64(y)
			if phase17 == 0 {
				a = math.NaN()
			}
			checksum += int64(cmp.Compare(a, b))
			if cmp.Less(a, b) {
				checksum += 3
			}
		case "choice":
			checksum += cmp.Or(int64(0), int64(0), x, y)
		case "string":
			a := []string{"", "alpha", "alpine", "é", "雪"}[phase5]
			b := []string{"", "alpha", "alpine", "é", "雪"}[phase3]
			checksum += int64(cmp.Compare(a, b))
			if cmp.Less(a, b) {
				checksum += 3
			}
		default:
			panic(mode)
		}
		x++
		if x == 129 {
			x = -128
		}
		y++
		if y == 97 {
			y = -96
		}
		phase17++
		if phase17 == 17 {
			phase17 = 0
		}
		phase5++
		if phase5 == 5 {
			phase5 = 0
		}
		phase3++
		if phase3 == 3 {
			phase3 = 0
		}
	}
	fmt.Println(time.Since(start).Nanoseconds(), checksum)
}
