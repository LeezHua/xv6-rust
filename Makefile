cc = gcc
rc = cargo

objs = main.o target/debug/libfun.so

main : $(objs)
	cc -o main $(objs)

main.o : main.c 
	$(cc) -c main.c -o main.o
target/debug/libfun.so : src/lib.rs
	$(rc) build

clean :
	-rm -rf target main.o main

