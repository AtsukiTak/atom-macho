samples/minimal.o: samples/minimal.s
	nasm -f macho64 samples/minimal.s

samples/minimal: samples/minimal.o
	ld -lSystem -L/Library/Developer/CommandLineTools/SDKs/MacOSX.sdk/usr/lib samples/minimal.o -o samples/minimal

samples/static.o: samples/static.s
	nasm -f macho64 samples/static.s

samples/static: samples/static.o
	ld -static samples/static.o

samples/minimal.gas.o: samples/minimal.gas.s
	as -o samples/minimal.gas.o samples/minimal.gas.s

samples/sample.o: samples/sample.c
	gcc -c -o samples/sample.o samples/sample.c
