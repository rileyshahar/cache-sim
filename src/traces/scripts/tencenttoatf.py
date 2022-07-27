"""Script for importing Systor '17 Traces from SNIA-IOTTA."""
import csv
import sys

TIME_UNIT = 1000000000

# Opens given LUN format file
with open(sys.argv[1], "r", encoding="utf-8") as source:
    reader = csv.reader(source,delimiter=',')
    # Sets the output file name as .atf
    output = f"{sys.argv[1]}.atf"
    with open(output, "w", newline="", encoding="utf-8") as result:
        writer = csv.writer(result)
        writer.writerow(("#Address", "Timestamp", "IOType", "Size", "Cost"))
        r1 = next(reader)
        time_start = float(r1[0])*TIME_UNIT
        writer.writerow((r1[1],int(float(r1[0])*TIME_UNIT-time_start),"R" if r1[3] == "0" else "W",r1[2],1))
        for r in reader:
            if(float(r[0]) < 0): time_start = r[0]
            writer.writerow((r[1],int(float(r[0])*TIME_UNIT-time_start),"R" if r[3] == "0" else "W",r[2],1))
