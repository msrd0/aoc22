import java.io.File

data class Move(
	val n: Int,
	val from: Int,
	val to: Int
)

data class Input(
	val stacks: MutableList<ArrayDeque<Char>>,
	val moves: MutableList<Move>
)

fun parseInput(): Input = run {
	val lines = File("input.txt").readLines()
	val stacks = mutableListOf<ArrayDeque<Char>>()
	val moves = mutableListOf<Move>()
	for (line in lines) {
		if (line.trim().startsWith('[')) {
			var stack = 0;
			var index = 1;
			while (index < line.length) {
				while (stacks.size <= stack) {
					stacks.add(ArrayDeque())
				}
				val ch = line[index]
				if (ch != ' ') {
					stacks[stack].addLast(ch)
				}

				index += 4
				stack += 1
			}
		}

		if (line.startsWith("move")) {
			val splits = line.split(' ');
			moves.add(Move(
				n = splits[1].toInt(),
				from = splits[3].toInt(),
				to = splits[5].toInt()
			))
		}
	}
	Input(stacks, moves)
}

// part 1

val input1 = parseInput()
for (move in input1.moves) {
	for (i in 0 until move.n) {
		input1.stacks[move.to-1].addFirst(input1.stacks[move.from-1].removeFirst())
	}
}
println(input1.stacks.joinToString("") { stack ->
	stack.first().toString()
})

// part 2

val input2 = parseInput()
for (move in input2.moves) {
	val crates = ArrayDeque<Char>(move.n)
	for (i in 0 until move.n) {
		crates.addFirst(input2.stacks[move.from-1].removeFirst())
	}
	for (crate in crates) {
		input2.stacks[move.to-1].addFirst(crate)
	}
}
println(input2.stacks.joinToString("") { stack ->
	stack.first().toString()
})
