"""Script for generating random atf files"""
import csv
import sys
import random
import os

elements = dict()
totalProb = 0
for i in range(0,400):
    weight = random.randint(0,8)
    elements[i] = weight
    totalProb += weight

output = os.path.join(os.pardir, f"{sys.argv[1]}.atf")
with open(output, "w", newline="", encoding="utf-8") as result:
    writer = csv.writer(result)
    writer.writerow(("#Address", "Timestamp", "IOtype", "Size", "Cost"))
    for i in range(3200):
        choice = random.randint(0,totalProb)
        for key in elements:
            if elements[key] < choice:
                choice -= elements[key]
            else:
                writer.writerow((key,i,"R",1,1))
                break
        
