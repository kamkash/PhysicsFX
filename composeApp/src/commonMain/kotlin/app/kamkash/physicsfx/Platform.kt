package app.kamkash.physicsfx

interface Platform {
    val name: String
}

expect fun getPlatform(): Platform