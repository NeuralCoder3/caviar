cargo run --release simplify data/prefix/evaluation_test.csv 1000 5000 10
# for applicable rules, 10k is the limit (7s)


# cargo run --release simplify data/prefix/evaluation_test.csv 1000 50000 10
# cargo run --release simplify data/prefix/evaluation_test.csv 1000 100000 10
# iterations nodes time

# cargo run --release simplify data/prefix/evaluation.csv 1000 50000 10 | tee results/iter_3.txt
# cargo run --release simplify data/prefix/evaluation.csv 1000 5000 10 | tee results/iter_4_cp.txt


# for mode in prove pulses npp pulses_npp; do
#     cargo run --release "$mode" data/prefix/evaluation.csv 1000 50000 10 | tee "results/${mode}.txt"
# done
cargo run --release prove data/prefix/evaluation.csv 1000 50000 10 | tee "results/prove.txt"
cargo run --release pulses data/prefix/evaluation.csv 1000 50000 10 2 | tee "results/pulses.txt"
cargo run --release npp data/prefix/evaluation.csv 1000 50000 10 | tee "results/npp.txt"
cargo run --release pulses_npp data/prefix/evaluation.csv 1000 50000 10 2 | tee "results/pulses_npp.txt"

# cargo run --release prove data/prefix/evaluation.csv 1000 50000 10 | tee results/iter_3.txt



# cargo run --release simplify data/prefix/evaluation_test.csv 1000 5000 10

# cargo run --release pulses data/prefix/evaluation_500.csv 1000 5000 10 2 | tee results/pulses_500.txt
# =COUNTIF(D:D,"=true")
# tmp/results_beh_2.csv
# 434

# 435 with cp


# cargo run --release simplify data/prefix/evaluation_500.csv 1000 5000 10 | tee results/simplify_500.txt
# =COUNTIF(P3:P502;"=0")+COUNTIF(P3:P502;"=1")
# 434

# A=5 eval echo '$A'

cargo run --release simplify data/own/simpl_50k.csv 1000 5000 10 | tee results/simplify_s50k_r_5k.txt
cargo run --release simplify data/own/simpl_50k.csv 1000 50000 10 | tee results/simplify_s50k_r_50k.txt

cargo run --release pulses data/own/pulse_50k.csv 1000 50000 10 2 | tee -a results/pulse_p50k_r_50k.txt
cargo run --release pulses data/own/pulse_50k.csv 1000 5000 10 2 | tee -a results/pulse_p50k_r_5k_v3.txt

RUST_BACKTRACE=full cargo run --release pulses data/own/pulse_50k.csv 1000 5000 10 2 2>&1 | tee results/pulse_p50k_r_5k_v9.txt
cargo run --release pulses data/own/pulse_50k_test2.csv 1000 5000 10 2 2>&1 | tee results/test.txt

(RUST_BACKTRACE=full SUFFIX=_p50k_r_5k_v14;cargo run --release pulses data/own/pulse_50k.csv 1000 5000 10 2 2>&1 | tee results/pulse$SUFFIX.txt; mv tmp/results_beh_2.csv tmp/results_beh_2_$SUFFIX.csv)


# =COUNTIF(E:E;"=0")+COUNTIF(E:E;"=1")
# simplify 50k r simpl 5k: 42
# simplify 50k r simpl 50k: 73
# pulse 50k r pulse 5k: 0
# pulse 50k r pulse 50k:
 
# pulse 50k r pulse 10k: 0 (14 saturated)

# pulse 50k r pulse 5k, cond CP: 3
# pulse 50k r pulse 10k (4s, 20s total), cond CP: 5
# pulse 50k r pulse 5k, cond CP, new rules: 15
# v9 : pulse 50k r pulse 5k, cond CP, new rules, 1k CP: 16
# v10: v9 + 1k CP rhs: 24
# v11: v10 + 500 CP rhs: 24
# v12: v10 + 2k CP rhs: 15
# v13: v10 + 500 CP rhs + mod-max rule: 37
# v14: v13 + new rules: 41
