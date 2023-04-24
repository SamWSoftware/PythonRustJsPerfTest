# Introduction

The aim of this is to test the performance of different languages.

This original started when some performance tests were published online. The magnitude of the difference seemed unlikely so I wanted to re-run the tests to verify the results. I also want to complete some further testing using other benchmarks.

## Initial Test setup

The first test came from the experiment published online. This case is a request to sum from 1 to 10,000 and then do that 25 times.

In all cases the total billed duration for the invocation was used.

All lambdas were deployed to eu-central-1. They were invoked in the console with this test object

`{
  "loop": 25,
  "count": 10000
}`

In Rust the key `loop` appears to be protected so it was changed to loops.

### Original Code

The exact code is included in this repo as a reference.

Having tested the code, it was found that the rust code was only doing the "sum" function once, not 25 times. This completely nullifies any of the original results, as the workload was not the same.

### Updated Code

Since the memory is under 1.8GB, the lambdas will be given a fraction of a CPU thread. This makes using multiprocessing unnecessary. The code has been changed to do series loops instead.

# Results

## Initial Testing

All of the test were run at 512mb and 128mb. The tests were done on the new code to ensure that the test were equivilent.

I do want to state that I am not a Python or Rust developer. There are many optimisations that have been suggested and I plan to open source this testing so that experts in each language can make the best case for their code.

### 512MB

| Language   | Average time (ms) | Average time (excluding cold start run) |
| ---------- | ----------------- | --------------------------------------- |
| Python     | 63.2              | 63.2                                    |
| Python v2  | 17.8              | 17.8                                    |
| Rust       | 62.6              | 52.5                                    |
| JavaScript | 68                | 61.25                                   |

From this you can see that there is a negligable difference between Rust and JavaScript. Python is by far the fastest language with the v2 updates. It's 4.6x faster than Rust.
Cold starts apear to have affected Rust (10ms = 19%) and and JavaScript (6.75ms = 11%)

### 128MB

| Language   | Average time (ms) | Average time (excluding cold start run) |
| ---------- | ----------------- | --------------------------------------- |
| Python     | 293               | 289.25                                  |
| Python v2  | 92.2              | 92.2                                    |
| Rust       | 295               | 292.5                                   |
| JavaScript | 425.6             | 409.75                                  |

At lower memory capcity the differences change, with Rust separating from JavaScript. Python is 320% faster than Rust, and JavaScript 44% slower again.

## Binary Trees

Summing numbers might not be the best way to test Lambda performance, so I decided to use benchmarks from [programming-language-benchmarks.vercel.app](https://programming-language-benchmarks.vercel.app/python-vs-rust).

Another advantage to this is that there are code solutions written by experts, so my personal coding ability isn't affecting the results.

### 512MB

| Language   | Average time (ms) |
| ---------- | ----------------- |
| Python     | 86596             |
| Rust       | 30068             |
| JavaScript | 28604             |

### 1792MB

| Language   | Average time (ms) |
| ---------- | ----------------- |
| Python     | 23692             |
| Rust       | 8079              |
| JavaScript | 1591.2            |

### 3584MB

| Language   | Average time (ms) |
| ---------- | ----------------- |
| Python     | 24142             |
| Rust       | 8346              |
| JavaScript | 1522              |

I then used the Lambda Powertuning tool to run each of the lambdas at a range of memory configurations. Each configuration was ran 5 times.

![Python Performance](./images/Python%20Lambda%20Performance.png)
![Rust Performance](./images/Rust%20Lambda%20Performance.png)
![JavaScript Performance](./images/JS%20Lambda%20Performce.png)

You can then compare the rust to JS performance graphs and see that Rust is outperformed at all memory configurations.
![Rust vs JS](./images/js-rust%20copy%202.png)

### Resuts

This test was very surprising, with JavaScript being the 5.4 times faster than Rust and 14.9 times faster than Python.

Given the first test showed the exact opposite ordering, I'm lead to believe that the speed of code has more to do with the specific task and the packages each language has access to rather than the language it's self.
