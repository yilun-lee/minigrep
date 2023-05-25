import numpy as np

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
    return np.array(MAT)

def mat_array(MAT):
    np.dot(MAT,MAT)

if __name__ == "__main__":
    # generate_rand_mat()
    MAT = np.random.rand(100,100)
    mat_array(MAT)


