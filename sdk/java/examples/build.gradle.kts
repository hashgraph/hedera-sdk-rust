plugins {
    `java-library`
}

repositories {
    mavenCentral()
}

java {
    sourceCompatibility = JavaVersion.VERSION_18
    targetCompatibility = JavaVersion.VERSION_18
}

dependencies {
    implementation(rootProject)
    implementation("com.fasterxml.jackson.core:jackson-databind:2.13.3")
}
