SOURCES=$(wildcard *.rs)

httpstat: ${SOURCES}
	rustc $<

debug: ${SOURCES}
	rustc -Z debug-info $<

clean:
	rm -f httpstat

.PHONY: clean debug
