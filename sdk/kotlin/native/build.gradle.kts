plugins {
    `java-library`
    `maven-publish`
}

group = "com.hedera.hashgraph"
version = "0.0.0"

java {
    sourceCompatibility = JavaVersion.VERSION_16
    targetCompatibility = JavaVersion.VERSION_16
}

tasks.jar {
    archiveBaseName.set("sdk-native")
}
