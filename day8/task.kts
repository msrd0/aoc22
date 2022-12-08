import java.io.File

data class Tree(
	val height: Int,
	var visible: Boolean?
) {
	constructor(height: Int): this(height, null)
}

fun List<List<Tree>>.print() {
	println("Trees:\n\t${joinToString("\n\t") { row ->
		row.joinToString(" ") { tree ->
			"${tree.height}" + when(tree.visible) {
				null -> "?"
				true -> "+"
				false -> "_"
			}
		}
	}}")
}

val trees = File("input.txt").readLines().map { line ->
	line.map { ch -> Tree(ch.toString().toInt()) }
}

// go through a row/column of trees
fun Iterable<Tree>.visit() = fold(-1) { height, tree ->
	if (height < tree.height) {
		tree.visible = true
		tree.height
	} else {
		height
	}
}

trees.forEach { row ->
	// look at all trees from left to right
	row.visit()

	// look at all trees from right to left
	row.reversed().visit()
}

for (i in 0 until trees[0].size) {
	trees.map { row -> row[i] }.visit()
	trees.reversed().map { row -> row[i] }.visit()
}

// trees.print()

val visible = trees.flatMap { row -> row.filter { tree -> tree.visible == true } }
println(visible.size)

// part 2

// get the score for all tree heights
fun Iterable<Tree>.score() = run {
	val score = IntArray(10) { 0 }
	map { tree ->
		val treeScore = score[tree.height]
		for (i in tree.height + 1.. 9) {
			score[i] += 1
		}
		for (i in 0 .. tree.height) {
			score[i] = 1
		}
		treeScore
	}
}

val scoreLtr: List<List<Int>> = trees.map { row -> row.score() }
val scoreRtl: List<List<Int>> = trees.map { row -> row.reversed().score().reversed() }
val scoreTtb: List<List<Int>> = (0 until trees[0].size).map { i ->
	trees.map { row -> row[i] }.score()
}
val scoreBtt: List<List<Int>> = (0 until trees[0].size).map { i ->
	trees.reversed().map { row -> row[i] }.score().reversed()
}

var best = 0
for (i in 0 until trees.size) {
	for (j in 0 until trees[i].size) {
		val score = scoreLtr[i][j] * scoreRtl[i][j] * scoreTtb[j][i] * scoreBtt[j][i]
		if (score > best) {
			best = score
		}
	}
}
println(best)
