import java.io.File

data class Directory(
	var size: Int,
	val subdirs: HashMap<String, Directory>,
	val parent: Directory?
) {
	fun totalSize(): Int = run {
		size + subdirs.values.map { it.totalSize() }.sum()
	}

	override fun toString() = "Directory(size=$size, subdirs=$subdirs)"

	fun ls(indent: String): String = run {
		subdirs.entries.joinToString("") { (name, dir) ->
			"$indent$name (${dir.size}, ${dir.totalSize()}):\n${dir.ls(indent + "  ")}"
		}
	}
}

val root = Directory(0, HashMap(), null)
var cwd = root

for (line in File("input.txt").bufferedReader().lines()) {
	if (line.startsWith('$')) {
		val split = line.substring(2).split(' ')
		if (split[0] == "cd") {
			when (split[1]) {
				"/" -> cwd = root
				".." -> cwd = cwd.parent!!
				else -> if (cwd.subdirs.containsKey(split[1])) {
					cwd = cwd.subdirs[split[1]]!!
				} else {
					cwd = Directory(0, HashMap(), cwd)
					cwd.parent!!.subdirs.put(split[1], cwd)
				}
			}
		} else if (split[0] == "ls") {
			if (cwd.size > 0) {
				println("WARN: possibly listing directory twice")
			}
		}
	} else if (line.startsWith("dir")) {
		val name = line.substring(4)
		if (!cwd.subdirs.containsKey(name)) {
			cwd.subdirs.put(name, Directory(0, HashMap(), cwd))
		}
	} else {
		val (size, _) = line.split(' ')
		cwd.size += size.toInt()
	}
}

println(root.ls(""))

var total = 0
fun visitDir(dir: Directory) {
	val size = dir.totalSize()
	if (size <= 100000) {
		total += size
	}
	for (subdir in dir.subdirs.values) {
		visitDir(subdir)
	}
}
for (subdir in root.subdirs.values) {
	visitDir(subdir)
}
println(total)

// part 2

var dirsize = 70000000
val freeSpace = 70000000 - root.totalSize()
val spaceNeeded = 30000000 - freeSpace
fun findDir(dir: Directory) {
	for (subdir in dir.subdirs.values) {
		val subdirsize = subdir.totalSize()
		if (subdirsize >= spaceNeeded && subdirsize < dirsize) {
			dirsize = subdirsize
		}
		findDir(subdir)
	}
}
findDir(root)
println(dirsize)
