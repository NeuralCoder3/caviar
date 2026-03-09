import re 
import sys
import os

file = "tmp/cp_rules_p50k_r_5k_v19.txt"

with open(file, "r") as f:
    lines = f.readlines()
    
cp_count = {}
    
current_id = None
for line in lines:
    match = re.match(r"^Id (\d+):", line)
    if match:
        current_id = match.group(1)
        continue
    
    # if current_id is not None:
    if line.startswith("cp_"):
        name, rest = line.split(":")
        expression, condition = rest.split("with")
        name = name.strip()
        expression = expression.strip()
        condition = condition.strip()
        key = (expression, condition)
        if key not in cp_count:
            cp_count[key] = 0
        cp_count[key] += 1
            
for (expression, condition), count in sorted(cp_count.items(), key=lambda x: x[1], reverse=True)[:20]:
    print(f"{count} occurrences of {expression} with condition {condition}")
    