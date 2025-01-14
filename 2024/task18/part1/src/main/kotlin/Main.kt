const val SIZE = 7
const val BYTE_COUNT = 12

enum class Cell {
    EMPTY,
    WALL,
}

fun main() {
    val file_path = "demo_input.txt"
//    val file_path = "input.txt"

    val data = object {}.javaClass.getResource(file_path).readText()
    println(data)

    val input: Array<Array<Cell>> = parse_lines(data)
    printMatrix(input)

    val output: Int = process_data(input)
    println("Output: $output")
}

fun process_data(matrix: Array<Array<Cell>>): Int {
    TODO()
}

fun parse_lines(data: String): Array<Array<Cell>> {
    val matrix = Array(SIZE) { Array(SIZE) { Cell.EMPTY } }

    data.split('\n')
        .take(BYTE_COUNT)
        .forEach { line ->
            val (x, y) = line.split(',').map { it.toInt() }
            matrix[y][x] = Cell.WALL
        }
    return matrix
}

fun printMatrix(matrix: Array<Array<Cell>>) {
    for (row in matrix) {
        println(row.joinToString("") {
            when (it) {
                Cell.EMPTY -> "."
                Cell.WALL -> "#"
            }
        })
    }
}