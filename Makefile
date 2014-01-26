httpstat: httpstat.rs nginx.rs
	rustc $<

clean:
	rm -f httpstat

.PHONY: clean
