SOURCES=src/httpstat.rs $(wildcard src/*.rs)

httpstat: ${SOURCES}
	rustc --opt-level=3 $< -o $@

386: httpstat.386

httpstat.386: ${SOURCES}
	# --link-args -static also can be added for statically linked binary
	rustc --opt-level=3 $< -o $@ --target=i686-unknown-linux-gnu

test: ${SOURCES}
	rustc --test -o $@ $<
	./test

debug: ${SOURCES}
	rustc -Z debug-info $< -o $@

clean:
	rm -f httpstat.386 httpstat debug test

.PHONY: clean debug 386 test
