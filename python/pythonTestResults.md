## OriginalCode

### 512 MB

| run | time (ms) | cold start |
| --- | --------- | ---------- |
| 1   | 399       | yes        |
| 2   | 344       | no         |
| 3   | 342       | no         |
| 4   | 340       | no         |
| 5   | 326       | no         |

## New Code

Since the memory is under 1.8GB, the lambdas will be given a fraction of a CPU thread. This makes using multiprocessing an unnecessary overhead. The code has been changed to do series loops instead.

### 512 MB

| run | time (ms) | cold start |
| --- | --------- | ---------- |
| 1   | 64        | yes        |
| 2   | 67        | no         |
| 3   | 63        | no         |
| 4   | 59        | no         |
| 5   | 63        | no         |

Average = 63.2ms

### 128 MB

| run | time (ms) | cold start |
| --- | --------- | ---------- |
| 1   | 308       | yes        |
| 2   | 282       | no         |
| 3   | 287       | no         |
| 4   | 295       | no         |
| 5   | 293       | no         |

Average = 293ms

# Reproducing this yourself

The python Lambdas are deployed using the Serveless Framework.

You can then deploy the code using `sls deploy`
