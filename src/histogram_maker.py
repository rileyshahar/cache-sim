#Call with python3 histogram_maker.py [filename w/ extension] [data format T/H/L] [(optional,H/L formats only) plot style L/P/C/G/else] [(optional) name of plot to show]
import matplotlib.pyplot as plt
import numpy as np
import csv
import sys


dataStyle = sys.argv[1]
with open(sys.argv[2], "r", encoding="utf-8") as source:
    reader = csv.reader(source,delimiter=',')
    if dataStyle == 'T':
        next(reader)
        if len(sys.argv) > 3:
            row = next(reader)
            while row[0] != sys.argv[3]:
                row = next(reader)
                
            (name, inf) = (row[0],row[11])
            sd_list = []
            for sd in row[12:]:
                value = int(sd[:sd.find(':')])
                num = int(sd[sd.find(':')+1:])
                sd_list.extend([value]*num)
            plt.hist(sd_list,bins=20)
            plt.axhline(y=int(inf),color='red',label="Infinities")
            plt.title(name)
            plt.xlabel("Stack Distance")
            plt.ylabel("Frequency")
            plt.legend()
            plt.show()
        else:
            for row in reader:
                (name, inf) = (row[0],row[11])
                sd_list = []
                for sd in row[12:]:
                    value = int(sd[:sd.find(':')])
                    num = int(sd[sd.find(':')+1:])
                    sd_list.extend([value]*num)
                plt.hist(sd_list,bins=20)
                plt.axhline(y=int(inf),color='red',label="Infinities")
                plt.title(name)
                plt.xlabel("Stack Distance")
                plt.ylabel("Frequency")
                plt.legend()
                plt.show()
        
    elif dataStyle == "H":
        next(reader)
        if len(sys.argv) > 4:
            row = next(reader)
            while row[0] != sys.argv[4]:
                row = next(reader)
            
            (name, entropy) = (row[0],row[1])
            item_list = []
            freq_list = []
            for entry in row[2:]:
                item_list.append(entry[:entry.find(':')])
                freq_list.append(int(entry[entry.find(':')+1:]))
            old_order = list(zip(item_list,freq_list))
            old_order.sort(key=lambda y: int(y[0]))
            new_order = [list(t) for t in zip(*old_order)]
            item_list = new_order[0]
            freq_list = new_order[1]
            if len(sys.argv) > 3 and sys.argv[3] == "L":
                item_list = [int(t) for t in item_list]
                plt.plot(item_list,freq_list,'b+')
                plt.axvline(x=0,color='red',label="Zero")
                margin = (max(item_list)-min(item_list))*0.02 + 1
                plt.xlim([min(item_list) - margin,max(item_list) + margin])
            elif len(sys.argv) > 3 and sys.argv[3] == "G":
                points = []
                for i in range(len(item_list)):
                    for j in range(freq_list[i]):
                        points.append(item_list[i])
                plt.hist(points,bins=100)
            else:
                plt.bar(item_list,freq_list)
            plt.title(sys.argv[2])
            plt.xlabel(name + ", Entropy: " + str(entropy))
            plt.ylabel("Frequency")
            plt.show()
        else:
            for row in reader:
                (name, entropy) = (row[0],row[1])
                item_list = []
                freq_list = []
                for entry in row[2:]:
                    item_list.append(entry[:entry.find(':')])
                    freq_list.append(int(entry[entry.find(':')+1:]))
                old_order = list(zip(item_list,freq_list))
                old_order.sort(key=lambda y: int(y[0]))
                new_order = [list(t) for t in zip(*old_order)]
                item_list = new_order[0]
                freq_list = new_order[1]
                if len(sys.argv) > 3 and sys.argv[3] == "L":
                    item_list = [int(t) for t in item_list]
                    plt.plot(item_list,freq_list,'b+')
                    plt.axvline(x=0,color='red',label="Zero")
                    margin = (max(item_list)-min(item_list))*0.02 + 1
                    plt.xlim([min(item_list) - margin,max(item_list) + margin])
                else:
                    plt.bar(item_list,freq_list)
                plt.title(sys.argv[2])
                plt.xlabel(name + ", Entropy: " + str(entropy))
                plt.ylabel("Frequency")
                plt.show()
    elif dataStyle == "L":
        next(reader)
        if len(sys.argv) > 4:
            row = next(reader)
            while row[0] != sys.argv[4]:
                row = next(reader)
            
            name,length = row[0],row[1]
            item_list = []
            freq_list = []
            for i in range(2,len(row)):
                item_list.append(str(i-1))
                freq_list.append(int(row[i]))
            if len(sys.argv) > 3 and sys.argv[3] == "P":
                item_list = [int(t) for t in item_list]
                for i in range(1,len(freq_list), -1):
                    freq_list[i] /= freq_list[i-1]
                freq_list[0] /= int(length)
                plt.plot(item_list,freq_list)
            elif len(sys.argv) > 3 and sys.argv[3] == "C":
                diff_list = []
                for i in range(0,len(freq_list)-1):
                    diff_list.append(freq_list[i+1] - freq_list[i])
                diff_list.append(0)
                diff_list2 = [0]
                for i in range(0,len(diff_list)-1):
                    diff_list2.append(diff_list[i+1] - diff_list[i])
                #diff_list2[0] = int(length) - sum(diff_list2)
                plt.bar(item_list,diff_list2)
            else:
                plt.bar(item_list,freq_list,width=1.0)
            plt.title(name + " (Legth: " + length + ")")
            plt.xlabel("Streak size")
            plt.ylabel("Frequency")
            plt.show()
        else:
            for row in reader:
                name,length = row[0],row[1]
                item_list = []
                freq_list = []
                for i in range(2,len(row)):
                    item_list.append(str(i-1))
                    freq_list.append(int(row[i]))
                if len(sys.argv) > 3 and sys.argv[3] == "P":
                    item_list = [int(t) for t in item_list]
                    for i in range(1,len(freq_list), -1):
                        freq_list[i] /= freq_list[i-1]
                    freq_list[0] /= int(length)
                    plt.plot(item_list,freq_list)
                else:
                    plt.bar(item_list,freq_list, width=1.0)
                plt.title(name + " (Legth: " + length + ")")
                plt.xlabel("Streak size")
                plt.ylabel("Frequency")
                plt.show()
