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

data class PathToVisit(val path: List<Point>, val checkPoint: Point)

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
    val (cache, bestPath) = bfsInit(graph, start, end)
    printMatrix(matrix, bestPath)

    val maxPathLength = bestPath.size - MIN_SAVE_PATH_LENGTH

    val paths = bfs(graph, start, end, maxPathLength, cache)
    paths.sortedDescending().forEach {
        println("saved moves: ${bestPath.size - it}")
//        printMatrix(matrix, it)
    }
    return paths.size
}

fun bfs(
    graph: Map<Point, GraphPosition>,
    start: Point,
    end: Point,
    maxPathLength: Int,
    cache: Map<Point, Int>
): List<Int> {
    val toVisit: Queue<PathToVisit> = LinkedList()
    toVisit.add(PathToVisit(mutableListOf(start), start))
    val paths: MutableList<Int> = mutableListOf()
    while (toVisit.isNotEmpty()) {
        val pathToVisit = toVisit.poll()
        val path = pathToVisit.path
        val point = pathToVisit.checkPoint

        if (path.size > maxPathLength) {
            continue
        }

        if (point == end) {
            paths.add(path.size)
            continue
        }

        val graphPosition = graph[point]!!

        for (neighbour in graphPosition.neighbours) {
            if (neighbour in path) {
                continue
            }
            val newPath = path + neighbour
            toVisit.add(PathToVisit(newPath, neighbour))
        }
        for (neighbour in graphPosition.cheatNeighbours) {
            if (neighbour in path || !cache.containsKey(neighbour)) {
                continue
            }
            val currentDistance = path.size
            val leftDistance = cache[neighbour]!!
            val totalDistance = currentDistance + leftDistance + 2
            if (totalDistance <= maxPathLength) {
                paths.add(totalDistance)
            }
        }
    }
    return paths
}

fun bfsInit(
    graph: Map<Point, GraphPosition>,
    start: Point,
    end: Point,
): Pair<Map<Point, Int>, List<Point>> {
    val toVisit: Queue<PathToVisit> = LinkedList()
    toVisit.add(PathToVisit(mutableListOf(start), start))
    val cache: MutableMap<Point, Int> = mutableMapOf()
    var bestPath: List<Point> = listOf()
    while (toVisit.isNotEmpty()) {
        val pathToVisit = toVisit.poll()
        val path = pathToVisit.path
        val point = pathToVisit.checkPoint

        if (point == end) {
            bestPath = path
            updateCache(path, cache)
            continue
        }

        val graphPosition = graph[point]!!
        for (neighbour in graphPosition.neighbours) {
            if (neighbour in path) {
                continue
            }
            val newPath = path + neighbour
            toVisit.add(PathToVisit(newPath, neighbour))
        }
    }
    return cache to bestPath
}

private fun updateCache(
    path: List<Point>,
    cache: MutableMap<Point, Int>
) {
    for ((index, point) in path.withIndex()) {
        val pointDistance = path.size - (index + 1)
        if (cache.getOrDefault(point, Int.MAX_VALUE) > pointDistance) {
            cache[point] = pointDistance
        }
    }
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
            addPointIfPossible(x - 1, y, matrix, neighbours)
            addPointIfPossible(x + 1, y, matrix, neighbours)
            addPointIfPossible(x, y - 1, matrix, neighbours)
            addPointIfPossible(x, y + 1, matrix, neighbours)
            addPointIfPossible(x - 2, y, matrix, cheatNeighbours)
            addPointIfPossible(x + 2, y, matrix, cheatNeighbours)
            addPointIfPossible(x, y - 2, matrix, cheatNeighbours)
            addPointIfPossible(x, y + 2, matrix, cheatNeighbours)
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
) {
    val width = matrix[0].size
    val height = matrix.size
    if (x < 0 || y < 0 || x >= width || y >= height || matrix[y][x] == Cell.WALL) {
        return
    }
    val checkedPoint = Point(x, y)
    neighbours.add(checkedPoint)
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

fun printMatrix(matrix: List<List<Cell>>, path: Collection<Point> = emptySet()) {
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
