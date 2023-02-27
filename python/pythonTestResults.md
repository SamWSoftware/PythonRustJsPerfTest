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

## Updated Code 2

This uses code optimisations suggested by the comunity.

v2 - use `list(range(count))` to generate the 10,000 item array.

### 512 MB

| run | time (ms) | cold start |
| --- | --------- | ---------- |
| 1   | 18        | yes        |
| 2   | 26        | no         |
| 3   | 25        | no         |
| 4   | 9         | no         |
| 5   | 10        | no         |

Average = 17.8ms

### 128 MB

| run | time (ms) | cold start |
| --- | --------- | ---------- |
| 1   | 94        | yes        |
| 2   | 93        | no         |
| 3   | 97        | no         |
| 4   | 83        | no         |
| 5   | 94        | no         |

Average = 92.2ms

# Reproducing this yourself

The python Lambdas are deployed using the Serveless Framework.

You can then deploy the code using `sls deploy`

# Better Benchmarks

Summing numbers might not be the best way to test Lambda performance, so I decided to use benchmarks from [programming-language-benchmarks.vercel.app](https://programming-language-benchmarks.vercel.app/python-vs-rust).

Another advantage to this is that there are code solutions written by experts, so my personal coding ability isn't affecting the results.

## Binary Trees - 1

https://programming-language-benchmarks.vercel.app/problem/binarytrees
Input = 18

### 512MB

| run | time (ms) | cold start |
| --- | --------- | ---------- |
| 1   | 86491     | yes        |
| 2   | 87134     | no         |
| 3   | 86373     | no         |
| 4   | 86450     | no         |
| 5   | 86532     | no         |

Average = 86596 ms = 68.569s

### 1792MB

| run | time (ms) | cold start |
| --- | --------- | ---------- |
| 1   | 23602     | yes        |
| 2   | 23779     | no         |
| 3   | 23661     | no         |
| 4   | 23748     | no         |
| 5   | 23671     | no         |

Average = 23692.2 ms = 23.6922 s

This is 3.655 times faster than with 512MB, but only 3.5 times the memory

### 3584MB

| run | time (ms) | cold start |
| --- | --------- | ---------- |
| 1   | 24142     | yes        |
| 2   | 24412     | no         |

From this we can see that there is almost no benefit (with this specific code) to having more than 1792MB ram. This is due to haing more vCPU cores, but not using any parallelisation.
