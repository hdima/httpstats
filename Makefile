httpstat: httpstat.rs nginx.rs
	rustc $<

debug: httpstat.rs nginx.rs
	rustc -Z debug-info $<

clean:
	rm -f httpstat

.PHONY: clean debug
