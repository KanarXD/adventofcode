package t20part1

import java.util.*

const val MIN_SAVE_PATH_LENGTH = 100

enum class Cell {
    EMPTY,
    WALL,
    START,
    END
}

data class Point(val x: Int, val y: Int)

data class PathToVisit(val path: Set<Point>, val checkPoint: Point, val canCheat: Int)

data class GraphPosition(val neighbours: List<Point>, val cheatNeighbours: List<Point>)

fun main() {
//    val file_path = "demo_input.txt"
    val file_path = "input.txt"

    val data = Thread.currentThread().contextClassLoader.getResource(file_path)!!.readText()
//    println(data)

    val matrix = parseLines(data)
    printMatrix(matrix)

    val output: Int = processData(matrix)
    println("Output: $output")
}


fun processData(matrix: List<List<Cell>>): Int {
    val start = findCell(matrix, Cell.START)
    val end = findCell(matrix, Cell.END)
    val graph = createGraph(matrix)
    println("start: $start, end: $end")
//    println(graph)
    val base_paths = bfs(graph, start, end, Int.MAX_VALUE, 0)
    val base_path = base_paths[0]
    printMatrix(matrix, base_path)

    val maxPathLength = base_path.size - MIN_SAVE_PATH_LENGTH

    val paths = bfs(graph, start, end, maxPathLength, 1)
    paths.forEach {
        println("saved moves: ${base_path.size - it.size}")
        printMatrix(matrix, it)
    }
    return paths.size
}

fun bfs(
    graph: Map<Point, GraphPosition>,
    start: Point,
    end: Point,
    maxPathLength: Int,
    cheatCount: Int
): List<Set<Point>> {
    val toVisit: Queue<PathToVisit> = LinkedList()
    toVisit.add(PathToVisit(linkedSetOf(start), start, cheatCount))
    val paths: MutableList<Set<Point>> = mutableListOf()
    while (toVisit.isNotEmpty()) {
        val pathToVisit = toVisit.poll()
        val path = pathToVisit.path
        val point = pathToVisit.checkPoint
        val canCheat = pathToVisit.canCheat

        if (path.size > maxPathLength) {
            continue
        }

        if (point == end) {
            paths.add(path)
            continue
        }

        val graphPosition = graph[point]!!

        if (canCheat > 0) {
            for (neighbour in graphPosition.cheatNeighbours) {
                if (neighbour in path) {
                    continue
                }
                val newPath = path + neighbour
                toVisit.add(PathToVisit(newPath, neighbour, canCheat - 1))
            }
        }
        for (neighbour in graphPosition.neighbours) {
            if (neighbour in path) {
                continue
            }
            val newPath = path + neighbour
            toVisit.add(PathToVisit(newPath, neighbour, canCheat))
        }
    }
    return paths
}

fun createGraph(matrix: List<List<Cell>>): Map<Point, GraphPosition> {
    val graph: MutableMap<Point, GraphPosition> = mutableMapOf()
    val width = matrix[0].size
    val height = matrix.size
    for (y in 0 until height) {
        for (x in 0 until width) {
            val point = Point(x, y)
            val neighbours: MutableList<Point> = mutableListOf()
            val cheatNeighbours: MutableList<Point> = mutableListOf()
            addPointIfPossible(x - 1, y, matrix, neighbours, cheatNeighbours)
            addPointIfPossible(x + 1, y, matrix, neighbours, cheatNeighbours)
            addPointIfPossible(x, y - 1, matrix, neighbours, cheatNeighbours)
            addPointIfPossible(x, y + 1, matrix, neighbours, cheatNeighbours)
            graph[point] = GraphPosition(neighbours, cheatNeighbours)
        }
    }
    return graph
}

private fun addPointIfPossible(
    x: Int,
    y: Int,
    matrix: List<List<Cell>>,
    neighbours: MutableList<Point>,
    cheatNeighbours: MutableList<Point>
) {
    val width = matrix[0].size
    val height = matrix.size
    if (x < 0 || y < 0 || x >= width || y >= height) {
        return
    }
    val checkedPoint = Point(x, y)
    if (matrix[y][x] == Cell.WALL) {
        cheatNeighbours.add(checkedPoint)
    } else {
        neighbours.add(checkedPoint)
    }
}

fun findCell(matrix: List<List<Cell>>, cell: Cell): Point {
    val width = matrix[0].size
    val height = matrix.size

    for (y in 0 until height) {
        for (x in 0 until width) {
            if (matrix[y][x] == cell) {
                return Point(x, y)
            }
        }
    }
    throw Exception("cell not found")
}

fun parseLines(data: String): List<List<Cell>> {
    val matrix: List<List<Cell>> =
        data.split("\n")
            .map { line ->
                line.map { c ->
                    when (c) {
                        '.' -> Cell.EMPTY
                        'S' -> Cell.START
                        'E' -> Cell.END
                        '#' -> Cell.WALL
                        else -> {
                            throw Exception("Unexpected character '$c'")
                        }
                    }
                }.toList()
            }.toList()
    return matrix
}

fun printMatrix(matrix: List<List<Cell>>, path: Set<Point> = emptySet()) {
    val width = matrix[0].size
    val height = matrix.size

    for (y in 0 until height) {
        for (x in 0 until width) {
            if (path.contains(Point(x, y))) {
                print("O")
            } else {
                when (matrix[y][x]) {
                    Cell.EMPTY -> print(".")
                    Cell.WALL -> print("#")
                    Cell.START -> print("S")
                    Cell.END -> print("E")
                }
            }
        }
        println()
    }
}
