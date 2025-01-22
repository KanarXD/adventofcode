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
//    val file_path = "demo_input.txt"
    val file_path = "input.txt"

    val data = Thread.currentThread().contextClassLoader.getResource(file_path)!!.readText()
//    println(data)

    val codes = parseLines(data)
    println("Codes: $codes")

    val output: Int = processData(codes)
    println("Output: $output")
}

fun processData(codes: List<String>): Int {
    val numericToDirection = findKeypadKeyToDirection(NUMERIC_KEYPAD, 0)
    val directionToDirection = findKeypadKeyToDirection(DIRECTIONAL_KEYPAD, 0)
    var sum = 0
    for (code in codes) {
        println("Code: $code")

        val codeList = code.map { KeypadKey.fromChar(it) }
        val s1 = sequenceToSequence(numericToDirection, codeList)
        val s2 = s1.map { sequence ->
//            println("s1: ${getSequenceString(sequence)}")
            sequenceToSequence(directionToDirection, sequence)
        }.flatten()

        val s3 = s2.map { sequence ->
//            println("s2: ${getSequenceString(sequence)}")
            sequenceToSequence(directionToDirection, sequence)
        }.flatten()

        val sequence = s3.minBy { it.size }
        val sequenceString = getSequenceString(sequence)
        val result = calculateResult(sequenceString, code)
        println("SequenceString: $sequenceString")
        println("Result: $result")
        sum += result
    }
    return sum
}

private fun getSequenceString(s3: List<KeypadKey>) = s3.map { it.toChar() }.joinToString("")

fun sequenceToSequence(
    mapper: Map<Pair<KeypadKey, KeypadKey>, List<List<KeypadKey>>>, baseSequence: List<KeypadKey>
): List<List<KeypadKey>> {
    val result: MutableList<List<KeypadKey>> = mutableListOf()

    val queue: Queue<List<List<KeypadKey>>> = LinkedList()
    mapper[KeypadKey.KeyA to baseSequence.first()]!!.forEach {
        queue.add(listOf(it))
    }

    while (queue.isNotEmpty()) {
        val sequences = queue.poll()!!
        val index = sequences.size

        if (baseSequence.size <= index) {
            result.add(sequences.flatten())
            continue
        }

        val leftKey = baseSequence[index - 1]
        val rightKey = baseSequence[index]

        if (leftKey == rightKey) {
            val newSequences: MutableList<List<KeypadKey>> = sequences.toMutableList()
            newSequences.add(listOf(KeypadKey.KeyA))
            queue.add(newSequences)
        } else {
            val key = leftKey to rightKey
            for (nextSequence in mapper[key]!!) {
                val newSequences: MutableList<List<KeypadKey>> = sequences.toMutableList()
                newSequences.add(nextSequence)
                queue.add(newSequences)
            }
        }
    }

    return result
}

fun findKeypadKeyToDirection(
    matrix: List<List<KeypadKey>>,
    maxDifference: Int
): Map<Pair<KeypadKey, KeypadKey>, List<List<KeypadKey>>> {
    val graph = generateGraph(matrix)
    val positionMap: MutableMap<Pair<Position, Position>, MutableList<List<KeypadKey>>> = mutableMapOf()
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
            val directionList = path.map { it.direction }.toList()
            if (positionMap.containsKey(keyPath)) {
                val positionList = positionMap[keyPath]!!
                val minSize = positionList.minOf { it.size }

                if (minSize + maxDifference >= directionList.size) {
                    positionList.removeIf {
                        it.size > directionList.size + maxDifference
                    }
                    positionList.add(directionList)
                }
            } else {
                positionMap[keyPath] = mutableListOf(directionList)
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

    val result: MutableMap<Pair<KeypadKey, KeypadKey>, List<List<KeypadKey>>> = mutableMapOf()
    positionMap.forEach { entry ->
        val keyLeft = positionToKey(entry.key.first, matrix)
        val keyRight = positionToKey(entry.key.second, matrix)
        val key = keyLeft to keyRight
        result[key] = entry.value.map { it.drop(1) + KeypadKey.KeyA }
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
            addPointIfPossible(x + 1, y, matrix, neighbours, KeypadKey.ArrowRight)
            addPointIfPossible(x, y + 1, matrix, neighbours, KeypadKey.ArrowDown)
            addPointIfPossible(x - 1, y, matrix, neighbours, KeypadKey.ArrowLeft)
            addPointIfPossible(x, y - 1, matrix, neighbours, KeypadKey.ArrowUp)
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
