package t2part1


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
    var sum = 0L
    for (range in ranges) {
        for (i in range.left..range.right) {
            if (isInvalid(i)) {
                sum += i
                println("$range - Invalid: $i")
            }
        }
    }

    return sum
}

fun isInvalid(number: Long): Boolean {
    val digits = number.toString()
    if (digits.length % 2 != 0) {
        return false
    }
    val middle = digits.length / 2
    val left = digits.take(middle)
    val right = digits.substring(middle)

    return left == right
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
