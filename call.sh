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


cargo run --release simplify data/own/simpl_50k.csv 1000 5000 10 | tee results/simplify_s50k_r_5k.txt
cargo run --release simplify data/own/simpl_50k.csv 1000 50000 10 | tee results/simplify_s50k_r_50k.txt

cargo run --release pulses data/own/pulse_50k.csv 1000 50000 10 2 | tee -a results/pulse_p50k_r_50k.txt

# =COUNTIF(E:E;"=0")+COUNTIF(E:E;"=1")
# simplify 50k r simpl 5k: 42
# simplify 50k r simpl 50k: 73
# pulse 50k r pulse 5k: 0
# pulse 50k r pulse 50k: