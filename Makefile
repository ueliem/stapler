build:
	mkdir -p build
	rustc src/main.rs -o build/stapler
clean:
	rm -rf build/*
	rmdir build/
run:
	./build/stapler
.PHONY: build
