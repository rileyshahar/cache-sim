import csv
import sys

counter = 0;
offsetMap = dict()
#Opens given LUN format file
with open(sys.argv[1], "r") as source:
    reader = csv.reader(source)
    next(reader)
    for r in reader:
        #associate each offset with a number
        if offsetMap.get(r[0]) is None:
            offsetMap[r[0]] = counter
            counter += 1


#open again to write
with open(sys.argv[1], "r") as source:
    reader = csv.reader(source)
    #Sets the output file name as .atf
    output = f"{sys.argv[1]}.atf"
    with open(output, "w",newline='') as result:
        writer = csv.writer(result)
        r1 = next(reader)
        writer.writerow((r1[0], r1[1], r1[2], r1[3], r1[4]))        
        for r in reader:
            writer.writerow((offsetMap.get(r[0]), r[1], r[2], r[3], r[4]))
