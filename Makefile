SOURCES=src/httpstats.rs $(wildcard src/*.rs) $(wildcard src/*/*.rs)

httpstats: ${SOURCES}
	rustc --opt-level=3 $< -o $@

686: httpstats.686

httpstats.686: ${SOURCES}
	# --link-args -static also can be added for statically linked binary
	rustc --opt-level=3 $< -o $@ --target=i686-unknown-linux-gnu

test: ${SOURCES}
	rustc --test -o $@ $<
	./test

debug: ${SOURCES}
	rustc -g $< -o $@

clean:
	rm -f httpstats.686 httpstats debug test

.PHONY: clean debug 686 test
