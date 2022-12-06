import java.io.File

val lines = File("input.txt").bufferedReader().lines()
val elves = mutableListOf<Int>(0)
for (line in lines) {
	if (line.isBlank()) {
		elves.add(0);
	} else {
		elves.add(elves.removeLast() + line.toInt())
	}
}
println(elves.max())

// part 2

elves.sortDescending()
println(elves.take(3).sum())
