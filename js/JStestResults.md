## Series Code

Since JavaScript was never tested originally we only have the series code to test

### 512 MB

| run | time (ms) | cold start |
| --- | --------- | ---------- |
| 1   | 95        | yes        |
| 2   | 73        | no         |
| 3   | 55        | no         |
| 4   | 64        | no         |
| 5   | 53        | no         |

Average = 68ms
Average without cold start = 61.25ms

### 128 MB

| run | time (ms) | cold start |
| --- | --------- | ---------- |
| 1   | 489       | yes        |
| 2   | 521       | no         |
| 3   | 393       | no         |
| 4   | 351       | no         |
| 5   | 374       | no         |

Average = 425.6ms
Average without cold start = 409.75ms

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
| 1   | 24148     | yes        |
| 2   | 49542     | no         |
| 3   | 23752     | no         |
| 4   | 22453     | no         |
| 5   | 23126     | no         |

Average = 28604 ms

### 1792MB

| run | time (ms) | cold start |
| --- | --------- | ---------- |
| 1   | 1834      | yes        |
| 2   | 1899      | no         |
| 3   | 1484      | no         |
| 4   | 1352      | no         |
| 5   | 1388      | no         |

Average = 1591.2 ms

This is 3.655 times faster than with 512MB, but only 3.5 times the memory

### 3584MB

| run | time (ms) | cold start |
| --- | --------- | ---------- |
| 1   | 1540      | yes        |
| 2   | 1504      | no         |

This shows that there are no performance benefits to going above 1792MB with JavaScript when the workload is cpu bound.
