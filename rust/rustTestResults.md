## OriginalCode

The issue with the original code is that, although claims were made of 100 concurrent threads, only one thread appears to run and output any results.

![Rust Single Thread](../images/RustSingleThread.png "Rust Single Thread")

This means that the lambda is not completing the same amount of work as the other languages, therefore it is unreasonable to be comparing them.

## New Code

This new code adds a for loop to ensure the calculation is completed 25 time. The number of loops and the value counted up to are both variablised so to prevent pre-calculation. There probably are better ways to do this but this was the only way I knew to compare equivilent processes.

### 512 MB

| run | time (ms) | cold start |
| --- | --------- | ---------- |
| 1   | 103       | yes        |
| 2   | 49        | no         |
| 3   | 61        | no         |
| 4   | 52        | no         |
| 5   | 48        | no         |

Average = 62.6ms
Average without cold start = 52.5ms

### 128 MB

| run | time (ms) | cold start |
| --- | --------- | ---------- |
| 1   | 305       | yes        |
| 2   | 296       | no         |
| 3   | 288       | no         |
| 4   | 294       | no         |
| 5   | 292       | no         |

Average = 295ms
Average without cold start = 292.5ms

# Reproducing this yourself

I followed [this tutorial](https://blog.logrocket.com/deploy-lambda-functions-rust/#upload-AWS-deploy-command) to get set up with the rust deployer Cargo.

`cargo lambda build`

To deploy to a specific region I used this command

`cargo lambda deploy newCode --region eu-central-1`

# Better Benchmarks

Summing numbers might not be the best way to test Lambda performance, so I decided to use benchmarks from [programming-language-benchmarks.vercel.app](https://programming-language-benchmarks.vercel.app/python-vs-rust).

Another advantage to this is that there are code solutions written by experts, so my personal coding ability isn't affecting the results.

## Binary Trees - 1

https://programming-language-benchmarks.vercel.app/problem/binarytrees
Input = 18

### 512MB

| run | time (ms) | cold start |
| --- | --------- | ---------- |
| 1   | 30172     | yes        |
| 2   | 29964     | no         |

Average = 30068 ms

### 1792MB

| run | time (ms) | cold start |
| --- | --------- | ---------- |
| 1   | 8036      | yes        |
| 2   | 8122      | no         |

Average = 8079 ms

### 3584MB

| run | time (ms) | cold start |
| --- | --------- | ---------- |
| 1   | 8399      | yes        |
| 2   | 8294      | no         |

Average = 8346 ms

This shows that the current rust code also doesn't take advantage of parallel processing
