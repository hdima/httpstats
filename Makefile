SOURCES=src/httpstats.rs $(wildcard src/*.rs)

httpstats: ${SOURCES}
	rustc --opt-level=3 $< -o $@

386: httpstats.386

httpstats.386: ${SOURCES}
	# --link-args -static also can be added for statically linked binary
	rustc --opt-level=3 $< -o $@ --target=i686-unknown-linux-gnu

test: ${SOURCES}
	rustc --test -o $@ $<
	./test

debug: ${SOURCES}
	rustc -Z debug-info $< -o $@

clean:
	rm -f httpstats.386 httpstats debug test

.PHONY: clean debug 386 test
