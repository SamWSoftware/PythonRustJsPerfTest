# Test setup

The aim of this is to test the performance of different languages.

The request is to sum from 1 to 10,000 and then do that 25 times.
In the original code this is

### Original Code

This is based on a test done by Noah. His exact code is used as a reference.

### Updated Code

Since the memory is under 1.8GB, the lambdas will be given a fraction of a CPU thread. This makes using multiprocessing unnecessary. The code has been changed to do series loops instead.

## TODO

[] Rust hasn't been deployed or tested yet
[] Try [python Cache](https://docs.python.org/3/library/functools.html)
[] Use [better benchmarks](https://programming-language-benchmarks.vercel.app/python-vs-rust)
[] Profile lambdas at different memory allocations
