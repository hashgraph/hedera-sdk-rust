plugins {
    `java-library`
}

repositories {
    mavenCentral()
}

java {
    sourceCompatibility = JavaVersion.VERSION_17
    targetCompatibility = JavaVersion.VERSION_17
}

dependencies {
    implementation(rootProject)
    implementation("com.fasterxml.jackson.core:jackson-databind:2.13.3")
}

tasks.addRule("Pattern: run<Example>: Runs an example.") {
    val taskPattern = this

    if (taskPattern.startsWith("run")) {
        val taskName = taskPattern.removePrefix("run") + "Example"

        task<JavaExec>(taskPattern) {
            mainClass.set(taskName)
            classpath = sourceSets["main"].runtimeClasspath
            standardInput = System.`in`
        }
    }
}
