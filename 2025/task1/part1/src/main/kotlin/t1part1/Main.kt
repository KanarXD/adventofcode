package t1part1

enum class Direction {
    LEFT, RIGHT
}

data class Operation(val direction: Direction, val value: Long)

fun main() {
//    val file_path = "demo_input.txt"
    val file_path = "input.txt"

    val data = Thread.currentThread().contextClassLoader.getResource(file_path)!!.readText()
    println(data)

    val parsedData = parseLines(data)
//    println("Codes: $parsedData")

    val output: Long = processData(parsedData)
    println("Output: $output")
}

fun processData(operations: List<Operation>): Long {
    var position = 50L
    var count = 0L

    for (operation in operations) {
        position = applyOperation(position, operation)
        println("Position: $position, Count: $count")
        if (position == 0L) {
            count++
        }
    }
    return count
}

private fun applyOperation(position: Long, operation: Operation): Long {
    val newPosition = when (operation.direction) {
        Direction.LEFT -> {
            position + 100 - operation.value
        }

        Direction.RIGHT -> {
            position + operation.value
        }
    }
    return newPosition % 100
}

fun parseLines(data: String): List<Operation> {
    return data.split("\n")
        .map {
            val direction = it[0]
            val value = it.substring(1).toLong()
            Operation(
                direction = if (direction == 'L') Direction.LEFT else Direction.RIGHT,
                value = value
            )
        }
}
