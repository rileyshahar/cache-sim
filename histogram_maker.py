import matplotlib.pyplot as plt
import numpy as np
import scipy.stats as st
import pandas as pd
import sys


traceOrNgram = sys.argv[1]
df = pd.read_csv(sys.argv[2],header=0)
if traceOrNgram == 'T':
    for index, row in df.iterrows():
        print(row)
        plt.hist(row, density=True, bins=100, label=index)
        mn, mx = plt.xlim()
        plt.xlim(mn, mx)
        plt.legend(loc="upper left")
        plt.ylabel("Probability")
        plt.xlabel("Distance")
        plt.title("Stack Distance");
        plt.show()
else:
    for index, row in df.iterrows():
        x = list(df.head())
        print(x)
        y = list(row)
        print(y)
        entr = y[1]
        d = {'col1':x[2:] , 'col2': y[2:]}
        d = pd.DataFrame.from_dict(d)
        d.plot.bar(x='col1', y='col2', rot=0)
        plt.ylabel("Frequency")
        plt.xlabel(f"Item   Entropy: {entr}")
        plt.title(str(list(row)[0]).split(",")[0])
        plt.show()
