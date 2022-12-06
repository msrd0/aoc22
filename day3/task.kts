import java.io.File

fun String.splitAt(index: Int): Pair<String, String> = run {
	this.substring(0..index-1) to this.substring(index)
}

fun Char.priority(): Int = run {
	when {
		this in ('a' .. 'z') -> this - 'a' + 1
		this in ('A' .. 'Z') -> this - 'A' + 27
		else -> throw IllegalArgumentException()
	}
}

val lines = File("input.txt").readLines()
val rucksacks = lines.map { line ->
	line.splitAt(line.length / 2)
}
val common = rucksacks.map { (first, second) ->
	val first = first.toSet()
	val second = second.toSet()
	first.intersect(second)
}
val prio = common.flatMap { it }.map { it.priority() }.sum()
println(prio)

// part 2

fun<T> Iterable<T>.skip(n: Int): Iterable<T> = run {
	this.withIndex().filter { (i, _) -> i >= n }.map { (_, value) -> value }
}

val groups = lines.chunked(3)
val badges = groups.map { group ->
	var possible = group.first().toSet()
	for (member in group.skip(1)) {
		possible = possible.intersect(member.toSet())
	}
	possible
}
val badgePrio = badges.flatMap { it }.map { it.priority() }.sum()
println(badgePrio)
