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

# (RUST_BACKTRACE=full SUFFIX=_p50k_r_5k_v26;rm -f tmp/cp_rules.txt;cargo run --release pulses data/own/pulse_50k.csv 1000 5000 10 2 2>&1 | tee results/pulse$SUFFIX.txt; mv tmp/results_beh_2.csv tmp/results_beh_2_$SUFFIX.csv;mv tmp/cp_rules.txt tmp/cp_rules$SUFFIX.txt)
# (RUST_BACKTRACE=full SUFFIX=_p50k_r_5k_v66;rm -f tmp/cp_rules.txt tmp/applied_rules.txt;cargo run --release --features='hotpath,hotpath-alloc' pulses data/own/pulse_50k.csv 1000 5000 10 2 2>&1 | tee results/pulse$SUFFIX.txt; mv tmp/results_beh_2.csv tmp/results_beh_2_$SUFFIX.csv;mv tmp/cp_rules.txt tmp/cp_rules$SUFFIX.txt;mv tmp/applied_rules.txt tmp/applied_rules$SUFFIX.txt)
# cargo run --features='hotpath,hotpath-alloc'

# (RUST_BACKTRACE=full SUFFIX=_p50k_r_5k_v15_all;cargo run --release pulses data/prefix/evaluation.csv 1000 5000 10 2 2>&1 | tee results/pulse$SUFFIX.txt; mv tmp/results_beh_2.csv tmp/results_beh_2_$SUFFIX.csv)

(RUST_BACKTRACE=full SUFFIX=_v80;rm -f tmp/cp_rules.txt tmp/applied_rules.txt;cargo run --release --features='hotpath,hotpath-alloc' pulses data/own/pulse_50k.csv 1000 5000 10 2 2>&1 | tee results/pulse$SUFFIX.txt | grep Start; mv tmp/results_beh_2.csv tmp/results$SUFFIX.csv;mv tmp/cp_rules.txt tmp/cp_rules$SUFFIX.txt;mv tmp/applied_rules.txt tmp/applied_rules$SUFFIX.txt)
cargo build --release --features='hotpath,hotpath-alloc'

# =COUNTIF(E:E;"=0")+COUNTIF(E:E;"=1")
# or ./check.sh v70
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
# v19: v14 + dedup cp: 32
# v20: v19, 100cp: 37
# v21: v19, 50cp: 41
# v22: v19, 75cp: 43
# v23: v19, 80cp: 38
# v24: v19, 60cp: 38
# v25=v22: v19, 75cp: 39
# v26: v22, cp on cps: 37
# v28: v22, cp on cps, 100cp: 37
# v29: v28, 7k nodes: 37
# v30: v29, 200 CP: 38
# v31: v29, 500 CP: 38
# v32: v28, 100 CP: 37
# v33: v28, 0 CP: 38
# v34: v28, 0 CP, no custom: 1
# v35: v28, 100 CP, no custom: 1
# v36: v28, 1000 CP, no custom: 1
# v37: v28, 1000 CP, no custom, 10k: 1
# v38: v28, 1000 CP, no custom, 7k, no cp on cp: 0
# v39: v28, 1000 CP, no custom, 5k, no cp on cp: 3
# v40: v39 + vec statt hashset: 1


cp: all_rules vs rules
hashset vs vec


40: 39+ vec statt hashset (1)
41: other compare (3)
42: no apps_vec (3)

43: all_rules 1000cp (does not finish 1 iteration)
44: all_rules 100cp (stuck after 7 expressions)
45: all_rules 50cp: 
46: all_rules 10cp: 3
47: all_rules 75_10cp: 4
49: all_rules 75_10cp, custom, (timing): 44
52: all_rules 75_20cp, custom, (timing): 39
53=49: all_rules 75_10cp, custom, (timing): 42
54=49: all_rules 75_10cp, custom, (timing): 38
56=49: all_rules 75_10cp, custom, (timing): --
60: all_rules 75_10cp, custom, cp condition replace, (timing): 60
61: 60+rc rules: 50
62=60: 62
63 = 61: 57
64 no custom: 36
65 no custom: ---
66 no custom: 37
67 15k (instead of 5k): ---
69 10k (instead of 5k): 58
70 15k (instead of 5k): 73
71 5k, 4s (per 2s iterations) = 2 rounds (instead of 5): 23
72 5k, 10s (per 2s iterations) = 5 rounds: 33
73 5k, 10s (per 3s iterations) = 3-4 rounds: 29
74 5k, 10s (per 1s iterations) = 10 rounds: 50
75 5k, 20s (per 2s iterations) = 10 rounds: 48
1008 50k 10s, 2s iter: 95
1011 50k 10s, 1s iter: 104

76, check inconsistency: (found 663 correct 0 sometimes (with 5 0 cp))
77, check inconsistency: (found 663 incorrect 1 sometimes (with 5 0 cp))
78, check inconsistency: (found 663 simpl incorrect 1 sometimes (with 5 0 cp))

# take rules both sided for cps / check in egraph
# take more rules for larger nodes (instead of 75_10)

cd ~/Documents/Projekte/synthesis/kbe_2.0/caviar_iter/results
grep "Start" results/pulse_p50k_r_5k_v53.txt


# cat results/pulse_v1008.txt | grep Start | tail -n 1


(RUST_BACKTRACE=full SUFFIX=_v79;rm -f tmp/cp_rules.txt tmp/applied_rules.txt;cargo run --release --features='hotpath,hotpath-alloc' pulses data/own/pulse_50k_cut1.csv 1000 5000 10 2 2>&1 | tee results/pulse$SUFFIX.txt | grep Start; mv tmp/results_beh_2.csv tmp/results$SUFFIX.csv;mv tmp/cp_rules.txt tmp/cp_rules$SUFFIX.txt;mv tmp/applied_rules.txt tmp/applied_rules$SUFFIX.txt) && ./check.sh v79