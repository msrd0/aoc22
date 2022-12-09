import java.io.File

fun Int.abs() = kotlin.math.abs(this)

data class Move(
	val direction: Char,
	val length: Int
) {
	companion object {
		fun parse(line: String): Move = run {
			val (direction, length) = line.split(' ')
			Move(direction[0], length.toInt())
		}
	}
}

data class Position(
	var x: Int,
	var y: Int
) {
	override fun toString() = "($x, $y)"
}

var head = Position(0, 0)
var tail = Position(0, 0)

fun moveTail() = run {
	val xdist = head.x - tail.x
	val xdiff = xdist.abs()

	val ydist = head.y - tail.y
	val ydiff = ydist.abs()

	if (xdiff >= 2 || ydiff >= 2) {
		if (xdiff >= 1) {
			tail.x += xdist / xdiff
		}
		if (ydiff >= 1) {
			tail.y += ydist / ydiff
		}
	}

	xdiff <= 2 && ydiff <= 2
}

val tailPos = HashSet<Position>()
tailPos.add(tail.copy())

fun print() {
	println(tailPos)
	for (y in minOf(head.y, tail.y, 0) .. maxOf(head.y, tail.y, 0)) {
	//for (y in -4 .. 0) {
		for (x in minOf(head.x, tail.x, 0) .. maxOf(head.x, tail.x, 0)) {
		//for (x in 0 .. 5) {
			if (head.x == x && head.y == y) {
				print('H')
			} else if (tail.x == x && tail.y == y) {
				print('T')
			} else if (x == 0 && y == 0) {
				print('s')
			} else if (Position(x, y) in tailPos) {
				print('#')
			} else {
				print('.')
			}
		}
		println()
	}
	println()
}

for (line in File("input.txt").bufferedReader().lines()) {
	//println("== $line ==")
	//println()

	val move = Move.parse(line)
	when (move.direction) {
		'R' -> head.x += move.length
		'L' -> head.x -= move.length
		'U' -> head.y -= move.length
		'D' -> head.y += move.length
	}

	//print()
	var done = false
	while (!done) {
		done = moveTail()
		tailPos.add(tail.copy())
		//print()
	}
}
println(tailPos.size)
