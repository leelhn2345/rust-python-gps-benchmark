# Rust-Python benchmark

Data processing script written in both **rust** and in **python**.

I would like to filter out elements with duplicated GPS data in `sample.json`.
Only the earliest timestamp of the duplicated GPS data is saved in the new json file.

For reasons unknown, the rust script took much longer to process.

Benchmark on my local machine:
    - python (~ 3,500 ms)
    - rust (~ 20,000 ms)
