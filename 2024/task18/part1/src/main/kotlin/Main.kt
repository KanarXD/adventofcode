import java.util.*

const val SIZE = 71
const val BYTE_COUNT = 1024

enum class Cell {
    EMPTY,
    WALL,
}

data class Point(val x: Int, val y: Int)

fun main() {
//    val file_path = "demo_input.txt"
    val file_path = "input.txt"

    val data = object {}.javaClass.getResource(file_path)!!.readText()
    println(data)

    val input: Array<Array<Cell>> = parseLines(data)
    printMatrix(input)

    val output: Int = processData(input)
    println("Output: $output")
}

fun processData(matrix: Array<Array<Cell>>): Int {
    val graph: Map<Point, List<Point>> = createGraph(matrix)
    val path = bfs(graph)
    printMatrix(matrix, path)
    return path.size - 1
}

fun bfs(graph: Map<Point, List<Point>>): List<Point> {
    val start = Point(0, 0)
    val end = Point(SIZE - 1, SIZE - 1)
    val toVisit: Queue<List<Point>> = LinkedList()
    val visited: MutableSet<Point> = mutableSetOf()

    toVisit.add(listOf(start))

    while (toVisit.isNotEmpty()) {
        val path = toVisit.poll()
        val point = path.last()

        if (point == end) {
            return path
        }

        for (neighbour in graph[point]!!) {
            if (neighbour in visited) {
                continue
            }
            visited.add(neighbour)
            toVisit.add(path + neighbour)
        }
    }
    throw Exception("path not found")
}

fun createGraph(matrix: Array<Array<Cell>>): Map<Point, List<Point>> {
    val graph: MutableMap<Point, List<Point>> = mutableMapOf()
    for (y in 0 until SIZE) {
        for (x in 0 until SIZE) {
            if (matrix[y][x] == Cell.WALL) {
                continue
            }
            val point = Point(x, y)
            val neighbours: MutableList<Point> = mutableListOf()
            addPointIfPossible(x - 1, y, matrix, neighbours)
            addPointIfPossible(x + 1, y, matrix, neighbours)
            addPointIfPossible(x, y - 1, matrix, neighbours)
            addPointIfPossible(x, y + 1, matrix, neighbours)
            graph[point] = neighbours
        }
    }
    return graph
}

private fun addPointIfPossible(x: Int, y: Int, matrix: Array<Array<Cell>>, neighbours: MutableList<Point>) {
    if (x < 0 || y < 0 || x >= SIZE || y >= SIZE || matrix[y][x] == Cell.WALL) {
        return
    }
    val checkedPoint = Point(x, y)
    neighbours.add(checkedPoint)
}

fun parseLines(data: String): Array<Array<Cell>> {
    val matrix = Array(SIZE) { Array(SIZE) { Cell.EMPTY } }

    data.split('\n')
        .take(BYTE_COUNT)
        .forEach { line ->
            val (x, y) = line.split(',').map { it.toInt() }
            matrix[y][x] = Cell.WALL
        }
    return matrix
}

fun printMatrix(matrix: Array<Array<Cell>>, path: List<Point> = emptyList()) {
    for (y in 0 until SIZE) {
        for (x in 0 until SIZE) {
            if (path.contains(Point(x, y))) {
                print("O")
            } else {
                when (matrix[y][x]) {
                    Cell.EMPTY -> print(".")
                    Cell.WALL -> print("#")
                }
            }
        }
        println()
    }
}
