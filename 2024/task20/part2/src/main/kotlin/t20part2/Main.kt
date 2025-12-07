package t20part2

import java.util.*

const val MIN_SAVE_PATH_LENGTH = 70
const val MAX_CHEAT_LENGTH = 20

enum class Cell {
    EMPTY,
    WALL,
    START,
    END
}

data class Point(val x: Int, val y: Int)

data class CheatPoint(val point: Point, val path: List<Point>)

data class GraphPosition(val neighbours: Set<Point>, val cheatNeighbours: Set<CheatPoint>)

fun main() {
    val file_path = "demo_input.txt"
//    val file_path = "input.txt"

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
    paths.sortedByDescending { it.size }.forEach {
        println("saved moves: ${bestPath.size - it.size}")
//        printMatrix(matrix, it)
    }
    return paths.size
}

fun bfs(
    graph: Map<Point, GraphPosition>,
    start: Point,
    end: Point,
    maxPathLength: Int,
    cache: Map<Point, List<Point>>
): List<List<Point>> {
    val toVisit: Queue<List<Point>> = LinkedList()
    toVisit.add(mutableListOf(start))
    val paths: MutableList<List<Point>> = mutableListOf()
    while (toVisit.isNotEmpty()) {
        val path = toVisit.poll()
        val point = path.last()

        if (path.size > maxPathLength) {
            continue
        }

        if (point == end) {
            paths.add(path)
            continue
        }

        val graphPosition = graph[point]!!

        for (neighbour in graphPosition.neighbours) {
            if (neighbour in path) {
                continue
            }
            val newPath = path + neighbour
            toVisit.add(newPath)
        }
        for ((neighbour, cheatPath) in graphPosition.cheatNeighbours) {
            val currentDistance = path.size
            val cheatedDistance = currentDistance + cheatPath.size
            if (cheatedDistance > maxPathLength || neighbour in path || !cache.containsKey(neighbour)) {
                continue
            }
            val leftPath = cache[neighbour]!!
            val totalDistance = cheatedDistance + leftPath.size
            if (totalDistance <= maxPathLength) {
                val fullPath = path + cheatPath + leftPath
                paths.add(fullPath)
            }
        }
    }
    return paths
}

fun bfsInit(
    graph: Map<Point, GraphPosition>,
    start: Point,
    end: Point,
): Pair<Map<Point, List<Point>>, List<Point>> {
    val toVisit: Queue<List<Point>> = LinkedList()
    toVisit.add(mutableListOf(start))
    val cache: MutableMap<Point, List<Point>> = mutableMapOf()
    var bestPath: List<Point> = listOf()
    while (toVisit.isNotEmpty()) {
        val path = toVisit.poll()
        val point = path.last()

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
            toVisit.add(newPath)
        }
    }
    return cache to bestPath
}

private fun updateCache(
    path: List<Point>,
    cache: MutableMap<Point, List<Point>>
) {
    val lastPath: MutableList<Point> = mutableListOf()
    for (point in path.reversed()) {
        val newPath: MutableList<Point> = lastPath.toMutableList()
        if (!cache.containsKey(point) || cache[point]!!.size > newPath.size) {
            cache[point] = newPath
        }
        lastPath.add(point)
    }
}

fun createGraph(matrix: List<List<Cell>>): Map<Point, GraphPosition> {
    val graph: MutableMap<Point, Set<Point>> = mutableMapOf()
    val globalGraph: MutableMap<Point, Set<Point>> = mutableMapOf()
    val resultGraph: MutableMap<Point, GraphPosition> = mutableMapOf()
    val width = matrix[0].size
    val height = matrix.size
    for (y in 0 until height) {
        for (x in 0 until width) {
            val neighbours: MutableSet<Point> = mutableSetOf()
            val allNeighbours: MutableSet<Point> = mutableSetOf()
            addPointIfPossible(x - 1, y, matrix, neighbours, allNeighbours)
            addPointIfPossible(x + 1, y, matrix, neighbours, allNeighbours)
            addPointIfPossible(x, y - 1, matrix, neighbours, allNeighbours)
            addPointIfPossible(x, y + 1, matrix, neighbours, allNeighbours)

            val point = Point(x, y)
            graph[point] = neighbours
            globalGraph[point] = allNeighbours
        }
    }
    graph.forEach { (point, neighbours) ->
        val cheatNeighbours: Set<CheatPoint> = bfsCheatPoints(matrix, globalGraph, point)
        resultGraph[point] = GraphPosition(neighbours, cheatNeighbours)
    }
    return resultGraph
}

fun bfsCheatPoints(
    matrix: List<List<Cell>>,
    globalGraph: Map<Point, Set<Point>>,
    start: Point
): Set<CheatPoint> {
    val toVisit: Queue<List<Point>> = LinkedList()
    val cheatPoints: MutableMap<Point, List<Point>> = mutableMapOf()
    val visited: MutableSet<Point> = mutableSetOf(start)
    for (neighbour in globalGraph[start]!!) {
        val (x, y) = neighbour
        if (matrix[x][y] == Cell.WALL) {
            visited.add(neighbour)
            toVisit.add(mutableListOf(neighbour))
        }
    }
    while (toVisit.isNotEmpty()) {
        val path = toVisit.poll()
        val point = path.last()
        val (x, y) = point

        if (path.size > MAX_CHEAT_LENGTH) {
            continue
        }

        if (matrix[y][x] != Cell.WALL) {
            if (!cheatPoints.containsKey(point) || cheatPoints[point]!!.size > path.size) {
                cheatPoints[point] = path.toList()
            }
//            continue
        }

        for (neighbour in globalGraph[point]!!) {
            if (neighbour in visited) {
                continue
            }
            val newPath = path + neighbour
            visited.add(neighbour)
            toVisit.add(newPath)
        }
    }
    return cheatPoints.map { (point, path) -> CheatPoint(point, path) }.toSet()
}

private fun addPointIfPossible(
    x: Int,
    y: Int,
    matrix: List<List<Cell>>,
    neighbours: MutableSet<Point>,
    allNeighbours: MutableSet<Point>,
) {
    val width = matrix[0].size
    val height = matrix.size
    if (x < 0 || y < 0 || x >= width || y >= height) {
        return
    }
    val checkedPoint = Point(x, y)
    allNeighbours.add(checkedPoint)
    if (matrix[y][x] != Cell.WALL) {
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
