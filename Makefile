all: bench

BENCHMARK = solve_bench
BASELINE = master
# FEATURES = matrix,rlu,lufact,klu
FEATURES = matrix,klu,csparse

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

clean:
	rm -rf ./target/criterion
