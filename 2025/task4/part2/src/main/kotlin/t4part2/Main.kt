package t4part2


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
    val toChoose = 12
    val batteries = bank.batteries
    val chosen = mutableListOf<Int>()
    var lastLeft = 0

    var maxLeft = 0
    while (chosen.size < toChoose) {

        val leftToCheckNext = toChoose - chosen.size - 1
        for (i in lastLeft + 1 until batteries.size - leftToCheckNext) {
            if (batteries[i] > batteries[maxLeft]) {
                maxLeft = i
            }
        }

        chosen.add(batteries[maxLeft])
        lastLeft = maxLeft
        maxLeft++
    }


    val resultString = chosen.joinToString("") { it.toString() }
    val result = resultString.toLong()
    println("Bank: $bank, chosen: $chosen, Result: $result")
    return result
}

fun parseLines(data: String): List<Bank> {
    return data.split("\n").map { line ->
        val batteries = line.map { i -> i.digitToInt() }
        Bank(batteries)
    }
}
