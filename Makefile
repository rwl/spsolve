all: bench

BENCHMARK = solve_bench
BASELINE = master
# FEATURES = matrix,rlu,lufact,klu
FEATURES = matrix,rlu,klu

.PHONY: bench
bench:
	# cargo criterion
	CRITERION_DEBUG=1 cargo bench --features $(FEATURES)

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
