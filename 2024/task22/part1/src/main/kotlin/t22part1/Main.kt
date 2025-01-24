package t22part1

const val ITERATIONS = 2000

fun main() {
//    val file_path = "demo_input.txt"
    val file_path = "input.txt"

    val data = Thread.currentThread().contextClassLoader.getResource(file_path)!!.readText()
//    println(data)

    val secrets = parseLines(data)
    println("Codes: $secrets")

    val output: Long = processData(secrets)
    println("Output: $output")
}

fun processData(secrets: List<Long>): Long {
    var sum = 0L
    for (secret in secrets) {
        sum += processSecret(secret)
    }
    return sum
}

fun processSecret(baseSecret: Long): Long {
    var secret = baseSecret
    for (i in 1..ITERATIONS) {
        val secretMultipliedBy64 = multiply(secret, 64)
        secret = mix(secret, secretMultipliedBy64)
        secret = prune(secret)
        val secretDividedBy32 = divide(secret, 32)
        secret = mix(secret, secretDividedBy32)
        secret = prune(secret)
        val secretMultipliedBy2048 = multiply(secret, 2048)
        secret = mix(secret, secretMultipliedBy2048)
        secret = prune(secret)
    }
//    println("Iteration: $ITERATIONS, baseSecret: $baseSecret, secret: $secret")
    return secret
}

fun prune(secret: Long): Long {
    return secret % 16777216
}

fun mix(secret: Long, valueToMixer: Long): Long {
    return secret.xor(valueToMixer)
}

fun divide(secret: Long, divider: Long): Long {
    return secret / divider
}

fun multiply(secret: Long, multiplier: Long): Long {
    return secret * multiplier
}

fun parseLines(data: String): List<Long> {
    return data.split("\n").map { it.toLong() }
}
