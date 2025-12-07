package t2part2


data class Range(val left: Long, val right: Long) {
    override fun toString(): String {
        return "[$left - $right]"
    }
}

fun main() {
//    val file_path = "demo_input.txt"
    val file_path = "input.txt"

    val data = Thread.currentThread().contextClassLoader.getResource(file_path)!!.readText()
    println(data)

    val parsedData = parseLines(data)
    println("Codes: $parsedData")

    val output: Long = processData(parsedData)
    println("Output: $output")
}

fun processData(ranges: List<Range>): Long {
    val ids = hashSetOf<Long>()
    for (range in ranges) {
        for (i: Long in range.left..range.right) {
            if (isInvalid(i)) {
                ids.add(i)
                println("$range - Invalid: $i")
            }
        }
    }

    return ids.sum()
}

fun parseLines(data: String): List<Range> {
    return data.split(",")
        .map {
            val (left, right) = it.split("-")
            Range(
                left.toLong(),
                right.toLong()
            )
        }
}

fun isInvalid(number: Long): Boolean {
    val digits = number.toString()
    if (digits.length <= 1) {
        return false
    }

    main@ for (patternLength in 1..digits.length / 2) {
        if (digits.length % patternLength != 0) {
            continue
        }
        val pattern = digits.take(patternLength)
        for (i in patternLength..digits.length - patternLength step patternLength) {
            val nextPattern = digits.substring(i, i + patternLength)
            if (nextPattern != pattern) {
                continue@main
            }
        }
        return true
    }

    return false
}
