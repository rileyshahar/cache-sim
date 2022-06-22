"""Script for importing Systor '17 Traces from SNIA-IOTTA."""
import csv
import sys

# Opens given LUN format file
with open(sys.argv[1], "r", encoding="utf-8") as source:
    reader = csv.reader(source)
    # Sets the output file name as .atf
    output = f"{sys.argv[1]}.atf"
    with open(output, "w", newline="", encoding="utf-8") as result:
        writer = csv.writer(result)
        for r in reader:
            # Removes the LUN col and changes to correct atf format
            writer.writerow((r[4], r[0], r[2], r[5], r[1]))
