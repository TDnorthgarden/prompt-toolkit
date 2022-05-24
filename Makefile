ROOT = $(shell git rev-parse --show-toplevel)

build:
	./scripts/build.sh -o $(ROOT)

release-build:
	./scripts/build.sh -o $(ROOT) -r

clean:
	./scripts/clean.sh $(ROOT)