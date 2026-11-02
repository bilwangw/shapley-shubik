# shapley-shubik

Calculates the Shapley-Shubik power index for a weighted voting game.

Input format:
```text
[quota, weight1, weight2, ...]
```

Run with `cargo run` and enter the array, or pipe it to the program. The output is an array of Shapley-Shubik values in voter order. Each value is the probability that the voter is pivotal in a uniformly random ordering.


Algorithm based on:
Castro, J., Gómez, D., & Tejada, J. (2009). Polynomial calculation of the Shapley value based on sampling. Selected Papers Presented at the Tenth International Symposium on Locational Decisions (ISOLDE X), 36(5), 1726–1730. https://doi.org/10.1016/j.cor.2008.04.004