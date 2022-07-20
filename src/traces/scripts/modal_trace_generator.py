"""Script for generating random atf files, with bimodal behavior"""
#Call using python3 modal_trace_generator.py [filename without extension] [mode type (f,s,d)] [number of elements] [length of mode]
#I have been using 200 elements and 1600 accesses per mode as a reference
#f = modal frequency; each mode draws from the same set of elements with different frequencies
#d = distinct elements; each mode has a completely different set of elements and frequencies
#s = subset; second mode uses a subset of the first mode's elements, with same relative frequencies
#outputs 3 atf files in the parent directory - each half and the full trace
import csv
import sys
import random
import os

random.seed()

elements1 = dict()
totalProb1 = 0

elements2 = dict()
totalProb2 = 0
    
if sys.argv[2] == "f":
    for i in range(0,int(sys.argv[3])):
        weight = random.randint(0,8)
        elements1[i] = weight
        totalProb1 += weight

    for i in range(0,int(sys.argv[3])):
        weight = random.randint(0,8)
        elements2[i] = weight
        totalProb2 += weight
        
elif sys.argv[2] == "d":
    for i in range(0,int(sys.argv[3])):
        weight = random.randint(0,8)
        elements1[i] = weight
        totalProb1 += weight

    for i in range(int(sys.argv[3]),2*int(sys.argv[3])):
        weight = random.randint(0,8)
        elements2[i] = weight
        totalProb2 += weight
        
elif sys.argv[2] == "s":
    for i in range(0,int(sys.argv[3])):
        weight = random.randint(0,8)
        elements1[i] = weight
        totalProb1 += weight
        
    elements2 = elements1.copy()
    totalProb2 = totalProb1
    for i in elements2:
        if random.random() < 0.5:
            weight = elements2[i]
            elements2[i] = 0
            totalProb2 -= weight

output = os.path.join(os.pardir, f"{sys.argv[1]}.atf")
output1 = os.path.join(os.pardir, f"{sys.argv[1]}-1.atf")
output2 = os.path.join(os.pardir, f"{sys.argv[1]}-2.atf")
with open(output, "w", newline="", encoding="utf-8") as result:
    with open(output1, "w", newline="", encoding="utf-8") as half1:
        with open(output2, "w", newline="", encoding="utf-8") as half2:
            writer = csv.writer(result)
            writer1 = csv.writer(half1)
            writer2 = csv.writer(half2)
            writer.writerow(("#Address", "Timestamp", "IOtype", "Size", "Cost"))
            writer1.writerow(("#Address", "Timestamp", "IOtype", "Size", "Cost"))
            writer2.writerow(("#Address", "Timestamp", "IOtype", "Size", "Cost"))
            for i in range(int(sys.argv[4])):
                choice = random.randint(0,totalProb1)
                for key in elements1:
                    if elements1[key] < choice:
                        choice -= elements1[key]
                    else:
                        writer.writerow((key,i,"R",1,1))
                        writer1.writerow((key,i,"R",1,1))
                        break
            for i in range(int(sys.argv[4]),2*int(sys.argv[4])):
                choice = random.randint(0,totalProb2)
                for key in elements2:
                    if elements2[key] < choice:
                        choice -= elements2[key]
                    else:
                        writer.writerow((key,i,"R",1,1))
                        writer2.writerow((key,i,"R",1,1))
                        break
        
