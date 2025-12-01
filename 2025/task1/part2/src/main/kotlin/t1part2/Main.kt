package t1part2

enum class Direction {
    LEFT, RIGHT
}

data class Operation(val direction: Direction, val value: Int) {
    override fun toString(): String {
        val dir = if (direction == Direction.LEFT) "L" else "R"
        return "$dir$value"
    }
}

fun main() {
//    val file_path = "demo_input.txt"
    val file_path = "input.txt"

    val data = Thread.currentThread().contextClassLoader.getResource(file_path)!!.readText()
    println(data)

    val parsedData = parseLines(data)
//    println("Codes: $parsedData")

    val output: Int = processData(parsedData)
    println("Output: $output")
}

fun processData(operations: List<Operation>): Int {
    var position = 50
    var count = 0

    for (operation in operations) {
        val prevPosition = position
        when (operation.direction) {
            Direction.LEFT -> {
                position -= operation.value
                if (position <= 0) {
                    count += 1 + (-position) / 100
                    if (prevPosition == 0) {
                        count -= 1
                    }
                }
                position = (100 + position % 100) % 100
            }

            Direction.RIGHT -> {
                position += operation.value
                count += position / 100
                position %= 100
            }
        }
        println("$operation -> position: $position, count: $count")
    }
    return count
}

private fun applyOperation(position: Int, operation: Operation): Int {
    return when (operation.direction) {
        Direction.LEFT -> {
            position + 100 - operation.value
        }

        Direction.RIGHT -> {
            position + operation.value
        }
    }
}

fun parseLines(data: String): List<Operation> {
    return data.split("\n")
        .map {
            val direction = it[0]
            val value = it.substring(1).toInt()
            Operation(
                direction = if (direction == 'L') Direction.LEFT else Direction.RIGHT,
                value = value
            )
        }
}
