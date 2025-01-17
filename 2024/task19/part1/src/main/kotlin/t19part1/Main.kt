package t19part1

fun main() {
//    val file_path = "demo_input.txt"
    val file_path = "input.txt"

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
            println("towel: $towel possible")
        } else {
            println("towel: $towel not possible")
        }
    }
    return sum
}

fun isTowelPossible(patterns: List<String>, towel: String): Boolean {
    if (towel.isEmpty()) {
        return true
    }
    for (pattern in patterns) {
        if (towel.indexOf(pattern) == 0) {
            val newTowel = towel.substring(pattern.length)
            if (isTowelPossible(patterns, newTowel)) {
                return true
            }
        }
    }
    return false
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
