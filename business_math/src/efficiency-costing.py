"""
Take csv of headers <entry, ideal, actual>, delimiter = ','
:entry = string, "RESULT" is reerved
:ideal = number | arithmetic expression
:actual = number | arithmetic expression

Calculates the efficiency of each entry
NOTE: This uses eval to evalulate the arithmetic expression, so be careful
"""

import pandas as pd
import json

def main(csv):
    csv = pd.read_csv(csv, encoding="utf-8", delimiter=",")
    tab = {}
    total_actual = 0
    total_ideal = 0
    for _index, row in csv.iterrows():
        entry = row["entry"]
        ideal = row["ideal"]
        actual = row["actual"]
        if type(ideal) == str:
            ideal = eval(ideal)
        if type(actual) == str:
            actual = eval(actual)
        tab[entry] = {"ideal": ideal, "actual": actual}
        total_actual += actual
        total_ideal += ideal

    efficiency_loss = 0
    for key, entry in tab.items():
        efficiency_loss +=  (entry["actual"] - entry["ideal"] )/total_actual
        tab[key] = {
            "ideal": entry["ideal"], 
            "actual": entry["actual"], 
            "efficiency_loss": (entry["actual"] - entry["ideal"] )/total_actual
        }
    tab["RESULT"] = {
        "total_ideal" : total_ideal,
        "total_actual" : total_actual,
        "total_efficiency_loss" : efficiency_loss,
        "efficiency": 1 - efficiency_loss
    }
    print(json.dumps(tab, indent=2))
    return

if __name__ == "__main__":
    import sys
    main(sys.argv[1])
    
