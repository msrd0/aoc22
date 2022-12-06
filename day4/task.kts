import java.io.File

fun String.parseRange(): Pair<Int, Int> = run {
	val (first, second) = split('-')
	first.toInt() to second.toInt()
}

val lines = File("input.txt").readLines()
val pairs = lines.map { line ->
	val (first, second) = line.split(',')
	first.parseRange() to second.parseRange()
}
val fullyContained = pairs.filter { (first, second) ->
	val firstRange = first.first .. first.second
	val secondRange = second.first .. second.second
	(first.first in secondRange && first.second in secondRange) ||
		(second.first in firstRange && second.second in firstRange)
}
println(fullyContained.size)

val partlyContained = pairs.filter { (first, second) ->
	val firstRange = first.first .. first.second
	val secondRange = second.first .. second.second
	(first.first in secondRange || first.second in secondRange) ||
		(second.first in firstRange || second.second in firstRange)
}
println(partlyContained.size)
