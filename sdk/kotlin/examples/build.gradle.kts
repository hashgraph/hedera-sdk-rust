plugins {
    id("org.jetbrains.kotlin.jvm") version "1.5.31"
    `java-library`
}

repositories {
    mavenCentral()
}

java {
    sourceCompatibility = JavaVersion.VERSION_16
    targetCompatibility = JavaVersion.VERSION_16
}

dependencies {
    implementation(rootProject)
    implementation("com.google.code.findbugs:jsr305:3.0.2")
    implementation(platform("org.jetbrains.kotlin:kotlin-bom"))
}

tasks.withType(org.jetbrains.kotlin.gradle.tasks.KotlinCompile::class).all {
    kotlinOptions {
        jvmTarget = "16"
    }
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
