SOURCES=src/httpstat.rs $(wildcard src/*.rs)

httpstat: ${SOURCES}
	rustc $< -o $@

386: httpstat.386

httpstat.386: ${SOURCES}
	# FIXME: Add also --link-args -static?
	rustc $< -o $@ --target=i686-unknown-linux-gnu

test: ${SOURCES}
	rustc --test -o $@ $<
	./test

debug: ${SOURCES}
	rustc -Z debug-info $< -o $@

clean:
	rm -f httpstat.386 httpstat debug test

.PHONY: clean debug 386 test
