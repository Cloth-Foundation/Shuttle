plugins {
	kotlin("jvm") version "2.4.20"
}

group = "cloth"
version = "1.0-SNAPSHOT"

repositories {
	mavenCentral()
}

dependencies {
	implementation(project(":Compiler"))
	implementation(platform("com.fasterxml.jackson:jackson-bom:2.21.1"))
	implementation("com.fasterxml.jackson.dataformat:jackson-dataformat-toml")
	testImplementation(platform("org.junit:junit-bom:5.10.0"))
	testImplementation(kotlin("test-junit5"))
	testRuntimeOnly("org.junit.platform:junit-platform-launcher")
}

// Execute configuration checks with native backend jars absent from the JVM classpath.
tasks.register<Test>("configurationTest") {
	workingDir(rootProject.projectDir)
	testClassesDirs = sourceSets.test.get().output.classesDirs
	classpath = sourceSets.test.get().runtimeClasspath.filter {
		!it.name.startsWith("llvm-") && !it.name.startsWith("javacpp-")
	}
	useJUnitPlatform()
	filter {
		includeTestsMatching("shuttle.ManifestTest")
		includeTestsMatching("shuttle.ProjectModelTest")
	}
}

tasks.check {
	dependsOn("configurationTest")
}

kotlin {
	jvmToolchain(25)
}

tasks.test {
	workingDir(rootProject.projectDir)
	useJUnitPlatform()
	jvmArgs("--enable-native-access=ALL-UNNAMED")
}
