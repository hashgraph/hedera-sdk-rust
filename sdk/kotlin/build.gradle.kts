plugins {
    id("org.jetbrains.kotlin.jvm") version "1.5.31"
    `java-library`
}

group = "com.hedera.hashgraph"
version = "0.0.0"

repositories {
    mavenCentral()
}

java {
    sourceCompatibility = JavaVersion.VERSION_16
    targetCompatibility = JavaVersion.VERSION_16
}

dependencies {
    implementation(project("native"))
    implementation(platform("org.jetbrains.kotlin:kotlin-bom"))
    implementation("org.jetbrains.kotlinx:kotlinx-coroutines-jdk8:1.6.2")
    implementation("org.jetbrains.kotlin:kotlin-stdlib-jdk8")
    implementation("com.github.jnr:jnr-ffi:2.2.12")
    implementation("com.fasterxml.jackson.core:jackson-core:2.13.3")
    api("com.fasterxml.jackson.core:jackson-annotations:2.13.3")
    implementation("com.fasterxml.jackson.core:jackson-databind:2.13.3")
}

tasks.withType(org.jetbrains.kotlin.gradle.tasks.KotlinCompile::class).all {
    kotlinOptions {
        jvmTarget = "16"
    }
}
