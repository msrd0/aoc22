import java.io.File

sealed abstract class RockPaperScissors {
	abstract fun score(): Int
}

object Rock: RockPaperScissors() {
	override fun score() = 1
}

object Paper: RockPaperScissors() {
	override fun score() = 2
}

object Scissors: RockPaperScissors() {
	override fun score() = 3
}

fun Char.toRockPaperScissors(): RockPaperScissors = run {
	when (this) {
		'A', 'X' -> Rock
		'B', 'Y' -> Paper
		'C', 'Z' -> Scissors
		else -> throw IllegalArgumentException()
	}
}

fun String.toRockPaperScissors(): Pair<RockPaperScissors, RockPaperScissors> = run {
	this[2].toRockPaperScissors() to this[0].toRockPaperScissors()
}

fun Pair<RockPaperScissors, RockPaperScissors>.score(): Int = run {
	var score = 0
	if (first is Rock) {
		if (second is Rock) {
			score = 3
		} else if (second is Scissors) {
			score = 6
		}
	} else if (first is Paper) {
		if (second is Paper) {
			score = 3
		} else if (second is Rock) {
			score = 6
		}
	} else {
		if (second is Scissors) {
			score = 3
		} else if (second is Paper) {
			score = 6
		}
	}
	score + first.score()
}

val lines = File("input.txt").readLines()
val scores: Iterable<Int> = lines.map { line ->
	line.toRockPaperScissors().score()
}
println(scores.sum())

// part 2

sealed abstract class Strategy {
	abstract fun answer(other: RockPaperScissors): RockPaperScissors
}

object Win: Strategy() {
	override fun answer(other: RockPaperScissors) = run {
		when (other) {
			is Rock -> Paper
			is Paper -> Scissors
			is Scissors -> Rock
			else -> throw IllegalStateException("THIS IS A FUCKING SEALED CLASS")
		}
	}
}

object Draw: Strategy() {
	override fun answer(other: RockPaperScissors) = other
}

object Loose: Strategy() {
	override fun answer(other: RockPaperScissors) = run {
		when (other) {
			is Rock -> Scissors
			is Paper -> Rock
			is Scissors -> Paper
			else -> throw IllegalStateException("THIS IS A FUCKING SEALED CLASS")
		}
	}
}

fun Char.toStrategy() = run {
	when (this) {
		'X' -> Loose
		'Y' -> Draw
		'Z' -> Win
		else -> throw IllegalArgumentException()
	}
}

val sum = lines.map { line ->
	val other = line[0].toRockPaperScissors()
	val strategy = line[2].toStrategy()
	val answer = strategy.answer(other)
	(answer to other).score()
}.sum()
println(sum)
