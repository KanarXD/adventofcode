package t4part1


data class Bank(val batteries: List<Int>) {
    override fun toString(): String {
        return batteries.joinToString(",")
    }
}

fun main() {
//    val file_path = "demo_input.txt"
    val file_path = "input.txt"

    val data = Thread.currentThread().contextClassLoader.getResource(file_path)!!.readText()
    println(data)

    val parsedData = parseLines(data)
    println("Codes: $parsedData")

    val output: Long = processData(parsedData)
    println("Output: $output")
}

fun processData(banks: List<Bank>): Long {
    var sum = 0L

    for (bank in banks) {
        sum += getVoltage(bank)
    }

    return sum
}

private fun getVoltage(bank: Bank): Long {
    val batteries = bank.batteries
    var maxLeft = 0
    var maxRight = batteries.size - 1
    var left = 0
    var right = batteries.size - 1
    while (left < maxRight) {
        if (batteries[left] > batteries[maxLeft]) {
            maxLeft = left
        }
        left++
    }
    while (right > maxLeft) {
        if (batteries[right] > batteries[maxRight]) {
            maxRight = right
        }
        right--
    }


    val resultString = batteries[maxLeft].toString() + batteries[maxRight].toString()
    val result = resultString.toLong()
    println("Bank: $bank, Max1: $maxLeft, Max2: $maxRight, Result: $result")
    return result
}

fun parseLines(data: String): List<Bank> {
    return data.split("\n")
        .map { line ->
            val batteries = line.map { i -> i.digitToInt() }
            Bank(batteries)
        }
}
