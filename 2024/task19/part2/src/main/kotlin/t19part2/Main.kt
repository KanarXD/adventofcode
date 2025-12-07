package t19part2

fun main() {
//    val file_path = "demo_input.txt"
    val file_path = "input.txt"

    val data = Thread.currentThread().contextClassLoader.getResource(file_path)!!.readText()
//    println(data)

    val (patterns, towels) = parseLines(data)
    println("patterns: $patterns")
    println("towels: $towels")

    val output: Long = processData(patterns, towels)
    println("Output: $output")
}


fun processData(patterns: List<String>, towels: List<String>): Long {
    var sum: Long = 0
    val cache = HashMap<String, Long>()
    for (towel in towels) {
        val possibilities = isTowelPossible(patterns, towel, cache)
        if (possibilities > 0) {
            sum += possibilities
            println("towel: $towel possible $possibilities times")
        } else {
            println("towel: $towel not possible")
        }
    }
    return sum
}

fun isTowelPossible(patterns: List<String>, towel: String, cache: HashMap<String, Long>): Long {
    if (cache.containsKey(towel)) {
        println("cache hit '$towel', value: ${cache[towel]!!}")
        return cache[towel]!!
    }
    if (towel.isEmpty()) {
        println("towel is possible")
        cache[towel] = 1
        return 1
    }
    var sum: Long = 0
    for (pattern in patterns) {
        if (towel.indexOf(pattern) == 0) {
            val newTowel = towel.substring(pattern.length)
            sum += isTowelPossible(patterns, newTowel, cache)
        }
    }
    cache[towel] = sum
    return sum
}

fun parseLines(data: String): Pair<List<String>, List<String>> {
    val (patternsString, towelsString) = data.split("\n\n")
    val patterns = patternsString.split(", ").sortedByDescending { it.length }
    val towels = towelsString.split("\n")
    return patterns to towels
}
