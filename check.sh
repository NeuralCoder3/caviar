# 1st argument: number (e.g. v70)
# find tmp/results_..._[number]...csv
# execute =COUNTIF(E:E;"=0")+COUNTIF(E:E;"=1")
# that is count 0 and 1s in column "best_expr"

if [ "$#" -ne 1 ]; then
    echo "Usage: $0 <number>"
    exit 1
fi
NAME=$1
FILE=$(find tmp -type f -name "results_*${NAME}*.csv")
if [ -z "$FILE" ]; then
    echo "No file found matching pattern: results_*${NAME}*.csv"
    exit 1
fi
echo "File: $FILE"
COUNT=$(awk -F, 'NR>1 {if ($5 == "0" || $5 == "1") count++} END {print count}' "$FILE")
# by column name:
# COUNT=$(awk -F, 'NR==1 {for (i=1; i<=NF; i++) if ($i == "best_expr") col=i} NR>1 {if ($col == "0" || $col == "1") count++} END {print count}' "$FILE")
echo "Count of 0 and 1 in column best_expr: $COUNT"
