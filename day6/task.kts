import java.io.File

fun findStart(input: String, n: Int): Int = run {
	var index = 0
	outer@ while (index + n < input.length) {
		val chars = mutableSetOf<Char>()
		for (i in 0 until n) {
			if (input[index + i] in chars) {
				index += 1
				continue@outer
			}
			chars.add(input[index + i])
		}
		return index + n
	}
	throw IllegalArgumentException()
}

val lines = File("input.txt").readLines()
val line = lines.first()
println(findStart(line, 4))

println(findStart(line, 14))
