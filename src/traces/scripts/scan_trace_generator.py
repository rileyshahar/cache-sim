"""Script for generating atf files of scans through lists"""
#Call using python3 scan_trace_generator.py [filename without extension] [number of elements] [number of scans]
#outputs an atf in the parent directory
import csv
import sys
import random
import os

'''
elements = dict()
totalProb = 0
for i in range(0,400):
    weight = random.randint(0,8)
    elements[i] = weight
    totalProb += weight
'''

output = os.path.join(os.pardir, f"{sys.argv[1]}.atf")
with open(output, "w", newline="", encoding="utf-8") as result:
    writer = csv.writer(result)
    writer.writerow(("#Address", "Timestamp", "IOtype", "Size", "Cost"))
    for i in range(int(sys.argv[2])*int(sys.argv[3])):
        key = i % int(sys.argv[2])
        writer.writerow((key,i,"R",1,1))
        
