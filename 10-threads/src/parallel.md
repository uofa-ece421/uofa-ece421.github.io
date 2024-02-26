# Parallel Programming

Parallel algorithms typically share data structures among multiple threads,
with the goal of solving a single problem in less time.

Data parallel algorithms can sometimes be classified *SIMD* (Single Instruction
Multiple Data) because each thread performs the same algorithm on a different
portion of the data.

Generally speaking, parallelism is hard, or to be more specific, achieving
<var>Speedup > 1</var> is hard.

> <var>Speedup<sub>N</sub> = T(sequential)/T(parallel<sub>N</sub>)</var>

Note that <var>T(sequential)</var> is typically _not_ <var>T(parallel<sub>1</sub></var>

**Amdahl's Law** (Speedup):

> <var>S<sub>latency</sub>(s) = [(1 - p) + p/s]<sup>-1</sup></var>

where
* <var>S<sub>latency</sub></var> is the theoretical speedup of the execution of the whole task
* <var>s</var> is the speedup of the part that can be parallelized
* <var>p</var> is the proportion that can be parallelized

For example, if 90% of a sequential program can be parallelized,
the best speedup you can get is <var>S</var> = 1/0.1 = 10 even if you managed to
get the parallel execution time to zero!

**Gustafson's Law** (Scaleup):

> <var>S = 1 + (N - 1)*p</var>

where
* <var>N</var> is the number of processors used

Gustafson's Law addresses the fact that most people increase the problem size
as they increase the number of processors.

For example, if you used 100 processors on a program that is 90% parallelizable,
Gustafson's Law predicts a scaleup <var>S</var> = 1 + (100 - 1)*0.9 = 90.1

**Inherent Costs of Parallelization**

In addition to the sequential part that can't be parallelized, you have to
minimize or amortize the expense of creating and coordinating threads:
* thread creation (`spawn`)
* thread communication, e.g. fork/join (barriers), events
* data integrity (locks)
* cache coherence and locality







