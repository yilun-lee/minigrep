import numpy as np
import pandas as pd

mat_file = '/Users/sox/CODE/minigrep/example/mat.csv'

def generate_rand_mat():
    MAT = np.random.rand(100,100)
    MAT2 = np.dot(MAT,MAT)
    
    pd.DataFrame(MAT).to_csv(mat_file, index=False, header = None)


def read_file():
    MAT = []
    with open(mat_file, 'r') as FF:
        for line in FF: 
            MAT.append( 
                [ 
                    float(i) for i in line.rstrip().split(",") 
                ]
            )
    return MAT

def mat_array(MAT):
    ll = len(MAT)
    product = [ [0.]*ll ]*ll  
    
    for i in range(ll):
        for j in range(ll):
            for k in range(ll):
                product[i][j] += MAT[i][k] * MAT[k][j]

if __name__ == "__main__":
    # generate_rand_mat()
    MAT = read_file()
    mat_array(MAT)


