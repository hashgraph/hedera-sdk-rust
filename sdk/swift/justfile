build:
    @ swift build -c release

alias fmt := format

format:
    @ swift format -i -p -r Sources Examples Package.swift

lint:
    @ swiftlint --strict --quiet
