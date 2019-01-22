rcp: main.rs
	rustc main.rs -o rcp

test: rcp
	./test.sh

clean:
	rm -f rcp tmp* *.o main
