package t21part1

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
    val numericToDirection: Map<KeypadKey, List<KeypadKey>> = findNumericToDirection()

    for (code in codes.take(1)) {
        val sequence: MutableList<KeypadKey> = mutableListOf()
        for (letter in code) {
            val key = KeypadKey.fromChar(letter)
            val keySequence = numericToDirection[key]!!
            sequence.addAll(keySequence)
        }
        println(sequence.map { it.toChar() }.joinToString(""))
    }

    return 0
}

fun findNumericToDirection(): Map<KeypadKey, List<KeypadKey>> {
    return mapOf(
        KeypadKey.Key0 to listOf(KeypadKey.ArrowDown, KeypadKey.ArrowLeft, KeypadKey.ArrowRight),
        KeypadKey.Key2 to listOf(KeypadKey.ArrowDown, KeypadKey.ArrowLeft, KeypadKey.ArrowRight),
        KeypadKey.Key9 to listOf(KeypadKey.ArrowDown, KeypadKey.ArrowLeft, KeypadKey.ArrowRight),
        KeypadKey.KeyA to listOf(KeypadKey.ArrowDown, KeypadKey.ArrowLeft, KeypadKey.ArrowRight),
    )
}

fun calculateResult(sequence: String, code: String): Int {
    val number = code.substring(0, code.length - 1).toInt()
    return sequence.length * number
}

fun parseLines(data: String): List<String> {
    return data.split("\n")
}
