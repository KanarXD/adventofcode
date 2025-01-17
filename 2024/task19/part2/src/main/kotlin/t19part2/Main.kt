package t19part2

import java.util.*

const val SIZE = 71

enum class Cell {
    EMPTY,
    WALL,
}

data class Point(val x: Int, val y: Int)

class PathNotFoundException(message: String) : Exception(message)

fun main() {
//    val file_path = "demo_input.txt"
    val file_path = "input.txt"

    val data = Thread.currentThread().contextClassLoader.getResource(file_path)!!.readText()
    println(data)

    val (matrix, bytes) = parseLines(data)
    printMatrix(matrix, bytes)

    val output: Int = processData(matrix, bytes)
    println("Output: $output, point: ${bytes[output]}")
}

fun processData(matrix: Array<Array<Cell>>, bytes: List<Point>): Int {
    for ((i, byte) in bytes.withIndex()) {
        matrix[byte.y][byte.x] = Cell.WALL
        try {
            val graph: Map<Point, List<Point>> = createGraph(matrix)
            val path = bfs(graph)
//            printMatrix(matrix, path)
        } catch (e: PathNotFoundException) {
            return i
        }
    }
    throw Exception("path is always found")
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
    throw PathNotFoundException("path not found")
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

fun parseLines(data: String): Pair<Array<Array<Cell>>, List<Point>> {
    val matrix = Array(SIZE) { Array(SIZE) { Cell.EMPTY } }
    val bytes = data.split('\n')
        .map { line ->
            val (x, y) = line.split(',').map { it.toInt() }
            Point(x, y)
        }.toCollection(LinkedList<Point>())

    return (matrix to bytes)
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
