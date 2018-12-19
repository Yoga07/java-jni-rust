#!/bin/bash

cargo build
mkdir -p build/classes/main
javac -d build/classes/main/ src/main/com/sample/jni/*.java
java -cp build/classes/main/ -Djava.library.path=target/debug com.sample.jni.Main
