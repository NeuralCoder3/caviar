import re
# import os
# import sys
from tqdm import tqdm

# file = "tmp/applied_rules_p50k_r_5k_v32.txt"
# file = "tmp/applied_rules_p50k_r_5k_v49.txt"
file = "tmp/applied_rules_p50k_r_5k_v52.txt"

# split ad ID \d+:
with open(file, "r") as f:
    content = f.read()
    
# matches = re.findall(r"ID (\d+):\nExpression: (.+?)\n( Iteration (\d+):\n(  EClass (\d+): .+?\n)*)*", content, re.DOTALL)

# for match in matches:
#     id = match[0]
#     expression = match[1]
#     print(f"ID: {id}")
#     print(f"Expression: {expression}")

# content = """ID 1:
# Expression: a + b
#  Iteration 1:
#   EClass 10: x = 1
#   EClass 11: y = 2
#  Iteration 2:
#   EClass 12: z = 3
# ID 2:
# Expression: c + d
#  Iteration 1:
#   EClass 13: w = 4"""

# 1. Match from one ID to the start of the next ID (or end of string)
expr_pattern = re.compile(r"^ID (\d+):\nExpression: ([^\n]+)\n(.*?)(?=^ID \d+:|\Z)", re.MULTILINE | re.DOTALL)

# 2. Match from one Iteration to the start of the next Iteration (or end of string)
iter_pattern = re.compile(r"^\s*Iteration (\d+):\n(.*?)(?=^\s*Iteration \d+:|\Z)", re.MULTILINE | re.DOTALL)

# 3. Match individual EClasses on a line-by-line basis
eclass_pattern = re.compile(r"^\s*EClass (\d+): (.*?)$", re.MULTILINE)



data = {}


# Iterate over Expressions
for expr_match in tqdm(list(expr_pattern.finditer(content))):
    expr_id = expr_match.group(1)
    expr_text = expr_match.group(2)
    expr_body = expr_match.group(3) # Contains all the Iterations text
    
    # print(f"Found Expression {expr_id}: {expr_text}")
    # data[expr_id] = {"expression": expr_text, "iterations": {}}
    data[expr_id] = {}
    
    # Iterate over Iterations WITHIN the current Expression block
    for iter_match in iter_pattern.finditer(expr_body):
        iter_id = iter_match.group(1)
        iter_body = iter_match.group(2) # Contains all the EClasses text
        
        # print(f"  -> Iteration {iter_id}")
        # data[expr_id]["iterations"][iter_id] = {"eclasses": {}}
        data[expr_id][iter_id] = {}
        
        # Iterate over EClasses WITHIN the current Iteration block
        for eclass_match in eclass_pattern.finditer(iter_body):
            eclass_id = eclass_match.group(1)
            eclass_val = eclass_match.group(2)
            
            # print(f"      - EClass {eclass_id}: {eclass_val}")
            # data[expr_id]["iterations"][iter_id]["eclasses"][eclass_id] = eclass_val
            data[expr_id][iter_id][eclass_id] = eclass_val
            
# count by rule
count = {}
for expr_id, expr_data in data.items():
    for iter_id, iter_data in expr_data.items():
        for eclass_id, eclass_val in iter_data.items():
            count[eclass_val] = count.get(eclass_val, 0) + 1
            
take = 50
for rule, c in sorted(count.items(), key=lambda x: x[1], reverse=True)[:take]:
    print(f"{c}x {rule}")

print("CP only:")
take = 50
items = count.items()
items = filter(lambda x: "cp_" in x[0].lower(), items)
items = sorted(items, key=lambda x: x[1], reverse=True)
for rule, c in items[:take]:
    print(f"{c}x {rule}")