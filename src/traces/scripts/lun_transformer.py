"""Script for importing Systor '17 Traces from SNIA-IOTTA."""
import csv
import sys

TIME_UNIT = 1000000

# Opens given LUN format file
with open(sys.argv[1], "r", encoding="utf-8") as source:
    reader = csv.reader(source)
    # Sets the output file name as .atf
    output = f"{sys.argv[1]}.atf"
    with open(output, "w", newline="", encoding="utf-8") as result:
        writer = csv.writer(result)
        h = next(reader)
        writer.writerow(("#" + h[4], h[0], h[2], h[5], h[1]))
        r1 = next(reader)
        timeStart = float(r1[0])*TIME_UNIT
        writer.writerow((r1[4], int(float(r1[0])*TIME_UNIT - timeStart), r1[2], r1[5], 1 if r1[1] == "" else r1[1]))
        for r in reader:
            # Removes the LUN col and changes to correct atf format
            # also gives a cost of 1 if no other cost given
            writer.writerow((r[4], int(float(r[0])*TIME_UNIT - timeStart), r[2], r[5], 1 if r[1] == "" else r[1]))
