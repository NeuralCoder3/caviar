# docker build -t neuralcoder/caviar-runner .

docker run --rm --init -v $(pwd)/results:/usr/src/caviar/results -v $(pwd)/tmp:/usr/src/caviar/tmp -v $(pwd)/data:/usr/src/caviar/data neuralcoder/caviar-runner \
/bin/bash -c "cd /usr/src/caviar; export RUST_BACKTRACE=full; \
export SUFFIX=_p50k_r_5k_v19; \
cargo run --release pulses data/own/pulse_50k.csv 1000 5000 10 2 2>&1 | tee results/pulse\$SUFFIX.txt; \
mv tmp/results_beh_2.csv tmp/results_beh_2_\$SUFFIX.csv"

