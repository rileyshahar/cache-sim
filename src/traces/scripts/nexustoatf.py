"""Script for importing Systor '17 Traces from SNIA-IOTTA."""
import csv
import sys

TIME_UNIT = 1000000000

# Opens given LUN format file
with open(sys.argv[1], "r", encoding="utf-8") as source:
    reader = csv.reader(source,delimiter='\t')
    # Sets the output file name as .atf
    output = f"{sys.argv[1]}.atf"
    with open(output, "w", newline="", encoding="utf-8") as result:
        writer = csv.writer(result)
        writer.writerow(("#Address", "Timestamp", "IOType", "Size", "Cost"))
        r1 = next(reader)
        r1 = list(filter(None,r1))
        time_start = float(r1[6])*TIME_UNIT
        writer.writerow((r1[0],int(float(r1[6])*TIME_UNIT-time_start),"R" if int(r1[3])%2 == 1 else "W",r1[1],1))
        for r in reader:
            r = list(filter(None,r))
            if(float(r[6]) < 0): time_start = r[6]
            writer.writerow((r[0],int(float(r[6])*TIME_UNIT-time_start),"R" if int(r[3])%2 == 1 else "W",r[1],1))
