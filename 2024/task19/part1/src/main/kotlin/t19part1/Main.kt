package t19part1

// 275

fun main() {
    val file_path = "demo_input.txt"
//    val file_path = "input.txt"

    val data = Thread.currentThread().contextClassLoader.getResource(file_path)!!.readText()
    println(data)

    val (patterns, towels) = parseLines(data)
    println("patterns: $patterns")
    println("towels: $towels")

    val output: Int = processData(patterns, towels)
    println("Output: $output")
}


fun processData(patterns: List<String>, towels: List<String>): Int {
    var sum = 0
    for (towel in towels) {
        if (isTowelPossible(patterns, towel)) {
            sum++
//            println("towel: $towel possible")
        } else {
//            println("towel: $towel not possible")
        }
    }
    return sum
}

fun isTowelPossible(patterns: List<String>, towel: String): Boolean {
    var position = 0
    while (position < towel.length) {
        val positionMove = findPattern(patterns, towel.substring(position))
        if (positionMove > 0) {
            position += positionMove
        } else {
            return false
        }
    }
    return true
}

private fun findPattern(
    patterns: List<String>,
    towelSubString: String,
): Int {
    for (pattern in patterns) {
        val index = towelSubString.indexOf(pattern)
        if (index == 0) {
            return pattern.length
        }
    }
    return -1
}


fun parseLines(data: String): Pair<List<String>, List<String>> {
    val (patternsString, towelsString) = data.split("\n\n")
    val patterns = patternsString.split(", ").sortedByDescending { it.length }
    val towels = towelsString.split("\n")
    return patterns to towels
}
