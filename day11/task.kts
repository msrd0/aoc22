import java.io.File

class Monkey(
	var items: ArrayDeque<Long>,
	var operation: (Long) -> Long,
	var test: Long,
	var monkey_true: Int,
	var monkey_false: Int,
	var inspections: Int = 0
) {
	companion object {
		fun dummy() = Monkey(
			items = ArrayDeque(),
			operation = { _ -> throw UnsupportedOperationException() },
			test = 0,
			monkey_true = -1,
			monkey_false = -1
		)
	}
}

val monkeys = mutableListOf<Monkey>()
for (line in File("input.txt").bufferedReader().lines()) {
	if (line.startsWith("Monkey")) {
		monkeys.add(Monkey.dummy())
	} else if (line.startsWith("  Starting items:")) {
		monkeys.last().items.addAll(line.substring(18).split(", ").map { it.toLong() })
	} else if (line.startsWith("  Operation: new = old")) {
		val (op, other) = line.substring(23).split(" ")
		val rhs = if (other == "old") {
			{ old: Long -> old }
		} else {
			{ _ -> other.toLong() }
		}
		monkeys.last().operation = when (op) {
			"+" -> { old -> old + rhs(old) }
			"*" -> { old -> old * rhs(old) }
			else -> throw IllegalArgumentException()
		}
	} else if (line.startsWith("  Test: divisible by ")) {
		monkeys.last().test = line.substring(21).toLong()
	} else if (line.startsWith("    If true: throw to monkey ")) {
		monkeys.last().monkey_true = line.substring(29).toInt()
	} else if (line.startsWith("    If false: throw to monkey ")) {
		monkeys.last().monkey_false = line.substring(30).toInt()
	} else if (line.isNotBlank()) {
		println("Unknown line: $line")
	}
}

// part 1
//val WORRY_LEVEL_ADJUST: (Long) -> Long = { item -> item / 3 }
//val ROUNDS: Int = 20

// part 2
val WORRY_LEVEL_ADJUST: (Long) -> Long = { item -> item }
val ROUNDS: Int = 10_000

val monkeyTotalDivisor = monkeys.map { m -> m.test }.fold(1L) { prod, test -> prod * test }
for (round in 0 until ROUNDS) {
	for (m in monkeys) {
		while (m.items.isNotEmpty()) {
			var item = m.items.removeFirst()
			m.inspections += 1
			item = WORRY_LEVEL_ADJUST(m.operation(item)) % monkeyTotalDivisor
			val monkey_idx = if (item % m.test == 0L) {
				m.monkey_true
			} else {
				m.monkey_false
			}
			monkeys[monkey_idx].items.addLast(item)
		}
	}
}

for ((i, m) in monkeys.withIndex()) {
	println("Monkey $i inspected items ${m.inspections} times.")
}
