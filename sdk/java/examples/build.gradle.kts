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
