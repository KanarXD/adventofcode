package t21part1

import java.util.*


enum class KeypadKey(val value: Char) {
    Key0('0'), Key1('1'), Key2('2'), Key3('3'), Key4('4'),
    Key5('5'), Key6('6'), Key7('7'), Key8('8'), Key9('9'),
    ArrowUp('^'), ArrowLeft('<'), ArrowRight('>'), ArrowDown('v'), KeyA('A'), UnusedKey('X');

    fun toChar(): Char = value

    companion object {
        fun fromChar(value: Char): KeypadKey {
            for (key in entries) {
                if (key.value == value) {
                    return key
                }
            }
            throw IllegalArgumentException("Unknown keypad key: $value")
        }
    }
}

data class Position(val x: Int, val y: Int)

data class DirectionPosition(val direction: KeypadKey, val position: Position)

data class Keypad(var currentPosition: Position, val keys: List<List<KeypadKey>>) {
    fun moveCursor(direction: KeypadKey) {
        val (x, y) = currentPosition
        val newPosition = when (direction) {
            KeypadKey.ArrowUp -> currentPosition.copy(y = y - 1)
            KeypadKey.ArrowDown -> currentPosition.copy(y = y + 1)
            KeypadKey.ArrowLeft -> currentPosition.copy(x = x - 1)
            KeypadKey.ArrowRight -> currentPosition.copy(x = x + 1)
            else -> throw Exception("Key '$direction' is not direction")
        }
        if (isValidPosition(newPosition)) {
            currentPosition = newPosition
        }
    }

    fun click(): KeypadKey {
        val (x, y) = currentPosition
        return keys[y][x]
    }

    private fun isValidPosition(position: Position): Boolean {
        val (x, y) = position
        return x >= 0 && x < keys[0].size && y >= 0 && y < keys.size && keys[y][x] != KeypadKey.UnusedKey
    }
}

val NUMERIC_KEYPAD = listOf(
    listOf(KeypadKey.Key7, KeypadKey.Key8, KeypadKey.Key9),
    listOf(KeypadKey.Key4, KeypadKey.Key5, KeypadKey.Key6),
    listOf(KeypadKey.Key1, KeypadKey.Key2, KeypadKey.Key3),
    listOf(KeypadKey.UnusedKey, KeypadKey.Key0, KeypadKey.KeyA),
)

val DIRECTIONAL_KEYPAD = listOf(
    listOf(KeypadKey.UnusedKey, KeypadKey.ArrowUp, KeypadKey.KeyA),
    listOf(KeypadKey.ArrowLeft, KeypadKey.ArrowDown, KeypadKey.ArrowRight),
)

fun main() {
    val file_path = "demo_input.txt"
//    val file_path = "input.txt"

    val data = Thread.currentThread().contextClassLoader.getResource(file_path)!!.readText()
//    println(data)

    val codes = parseLines(data)
    println("Codes: $codes")

    val output: Int = processData(codes)
    println("Output: $output")
}

fun processData(codes: List<String>): Int {
    val numericToDirection = findKeypadKeyToDirection(NUMERIC_KEYPAD)
    val directionToDirection = findKeypadKeyToDirection(DIRECTIONAL_KEYPAD)
    var sum = 0
    for (code in codes.take(1)) {
        println("Code: $code")
        val sequence: MutableList<KeypadKey> = mutableListOf()
        var lastKey = KeypadKey.KeyA
        for (letter in code) {
            val key = KeypadKey.fromChar(letter)
            val keySequence = numericToDirection[lastKey to key]!!
            sequence.addAll(keySequence)
            lastKey = key
        }

        val sequenceString = sequence.map { it.toChar() }.joinToString("")
        val result = calculateResult(sequenceString, code)
        println("SequenceString: $sequenceString")
        println("Result: $result")
        sum += result
    }
    return sum
}

fun findKeypadKeyToDirection(matrix: List<List<KeypadKey>>): Map<Pair<KeypadKey, KeypadKey>, List<KeypadKey>> {
    val graph = generateGraph(matrix)
    val positionMap: MutableMap<Pair<Position, Position>, List<KeypadKey>> = mutableMapOf()
    val queue: Queue<List<DirectionPosition>> = LinkedList()
    val width = matrix[0].size
    val height = matrix.size
    for (y in 0 until height) {
        for (x in 0 until width) {
            val position = Position(x, y)
            if (matrix[y][x] != KeypadKey.UnusedKey) {
                queue.add(mutableListOf(DirectionPosition(KeypadKey.UnusedKey, position)))
            }
        }
    }

    while (queue.isNotEmpty()) {
        val path = queue.poll()
        val basePosition = path.first().position
        val directionPosition = path.last()
        val position = directionPosition.position

        if (basePosition != position) {
            val keyPath = basePosition to position
            if (!positionMap.containsKey(keyPath) || positionMap[keyPath]!!.size > path.size) {
                positionMap[keyPath] = path.map { it.direction }.toList()
            }
        }

        val pathPositions = path.map { it.position }
        for (neighborDirectionPosition in graph[position]!!) {
            if (neighborDirectionPosition.position in pathPositions) {
                continue
            }
            val newPath = path + neighborDirectionPosition
            queue.add(newPath)
        }
    }

    val result: MutableMap<Pair<KeypadKey, KeypadKey>, List<KeypadKey>> = mutableMapOf()
    positionMap.forEach { entry ->
        val keyLeft = positionToKey(entry.key.first, matrix)
        val keyRight = positionToKey(entry.key.second, matrix)
        val key = keyLeft to keyRight
        result[key] = entry.value.drop(1) + KeypadKey.KeyA
    }
    return result
}

fun generateGraph(matrix: List<List<KeypadKey>>): Map<Position, List<DirectionPosition>> {
    val graph: MutableMap<Position, List<DirectionPosition>> = mutableMapOf()
    val width = matrix[0].size
    val height = matrix.size
    for (y in 0 until height) {
        for (x in 0 until width) {
            val position = Position(x, y)
            val neighbours: MutableList<DirectionPosition> = mutableListOf()
            addPointIfPossible(x - 1, y, matrix, neighbours, KeypadKey.ArrowLeft)
            addPointIfPossible(x + 1, y, matrix, neighbours, KeypadKey.ArrowRight)
            addPointIfPossible(x, y - 1, matrix, neighbours, KeypadKey.ArrowUp)
            addPointIfPossible(x, y + 1, matrix, neighbours, KeypadKey.ArrowDown)
            graph[position] = neighbours
        }
    }
    return graph
}

fun addPointIfPossible(
    x: Int, y: Int, matrix: List<List<KeypadKey>>, neighbours: MutableList<DirectionPosition>, direction: KeypadKey
) {
    val width = matrix[0].size
    val height = matrix.size
    if (x < 0 || y < 0 || x >= width || y >= height || matrix[y][x] == KeypadKey.UnusedKey) {
        return
    }
    val position = Position(x, y)
    neighbours.add(DirectionPosition(direction, position))
}

fun calculateResult(sequence: String, code: String): Int {
    val number = code.substring(0, code.length - 1).toInt()
    return sequence.length * number
}

fun parseLines(data: String): List<String> {
    return data.split("\n")
}

fun positionToKey(position: Position, matrix: List<List<KeypadKey>>): KeypadKey {
    return matrix[position.y][position.x]
}
