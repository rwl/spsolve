all: bench

BENCHMARK = solve_bench
BASELINE = master
# FEATURES = matrix,rlu,lufact,klu,csparse,rsparse
FEATURES = matrix,csparse,rsparse

.PHONY: bench
bench:
	# cargo criterion
	# CRITERION_DEBUG=0 cargo bench --features $(FEATURES)
	cargo bench --features $(FEATURES)

.PHONY: save
save:
	cargo bench --bench $(BENCHMARK) --features $(FEATURES) -- --save-baseline $(BASELINE)

.PHONY: baseline
baseline:
	cargo bench --bench $(BENCHMARK) --features $(FEATURES) -- --baseline $(BASELINE)

.PHONY: setup
setup:
	cargo install cargo-criterion
	go install github.com/google/pprof@latest


spsolve.profile:
	cargo run --features matrix,cpuprofiler,rlu --bin spsolve

.PHONY: pprof
pprof: spsolve.profile
	pprof -http=:8080 target/debug/spsolve spsolve.profile


clean:
	rm -f spsolve.profile

purge:
	rm -rf ./target/criterion
