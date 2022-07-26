"""Script for importing Systor '17 Traces from SNIA-IOTTA."""
import csv
import sys

TIME_UNIT = 1000000000

# Opens given LUN format file
with open(sys.argv[1], "r", encoding="utf-8") as source:
    reader = csv.reader(source,delimiter=' ')
    # Sets the output file name as .atf
    output = f"{sys.argv[1]}.atf"
    with open(output, "w", newline="", encoding="utf-8") as result:
        writer = csv.writer(result)
        writer.writerow(("#Address", "Timestamp", "IOType", "Size", "Cost"))
        r1 = next(reader)
        r1 = list(filter(None,r1))
        time_start = float(r1[3])*TIME_UNIT
        if(len(r1) > 10):
            writer.writerow((r1[7],int(float(r1[3])*TIME_UNIT-time_start),"R" if 'R' in r1[6] else "W",r1[9],1))
        for r in reader:
            if("CPU" in r[0]): break
            r = list(filter(None,r))
            if(float(r[3]) < 0): time_start = r[3]
            if(len(r) > 10):
                writer.writerow((r[7],int(float(r[3])*TIME_UNIT-time_start),"R" if "R" in r[6] else "W",r[9],1))
