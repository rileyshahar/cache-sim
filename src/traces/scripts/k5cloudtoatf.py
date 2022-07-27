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
        time_start = float(r1[1])*TIME_UNIT
        writer.writerow((r1[3],int(float(r1[1])*TIME_UNIT-time_start),r1[2],r1[4],1))
        for r in reader:
            if(int(float(r[1])*TIME_UNIT-time_start) < 0): time_start = r[1]*TIME_UNIT
            writer.writerow((r[3],int(float(r[1])*TIME_UNIT-time_start),r[2],r[4],1))
