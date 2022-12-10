import java.io.File

var cycle = 0
var x = 1
var crt_x = 0
var total = 0

fun finishCycle() {
	if (crt_x >= x - 1 && crt_x <= x + 1) {
		print("##")
	} else {
		print("  ")
	}
	cycle += 1
	crt_x = (crt_x + 1) % 40
	if (cycle % 40 == 20) {
		total += cycle * x
	}
	if (crt_x == 0) {
		println()
	}
}

for (line in File("input.txt").bufferedReader().lines()) {
	finishCycle()
	if (line.startsWith("addx")) {
		finishCycle()
		val rhs = line.substring(5).toInt()
		x += rhs
	}
}
println(total)
