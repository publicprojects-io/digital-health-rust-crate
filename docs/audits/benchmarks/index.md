# Benchmarks

## Table of Contents

- [Benchmark Results](#benchmark-results)
    - [ratios](#ratios)
    - [money](#money)
    - [percentile](#percentile)

## Benchmark Results

### ratios

|        | `readmission_rate`          | `net_promoter_score`           | `roi_percent`                  | `telehealth_visit_rate`          | `sus_score`                    | `zero_denominator_none`           |
|:-------|:----------------------------|:-------------------------------|:-------------------------------|:---------------------------------|:-------------------------------|:--------------------------------- |
|        | `0.72 ns` (✅ **1.00x**)     | `0.94 ns` (❌ *1.31x slower*)   | `0.77 ns` (✅ **1.07x slower**) | `0.81 ns` (❌ *1.13x slower*)     | `1.52 ns` (❌ *2.12x slower*)   | `0.61 ns` (✅ **1.19x faster**)    |

### money

|        | `bed_day_cost_avoided`           |
|:-------|:-------------------------------- |
|        | `2.59 ns` (✅ **1.00x**)          |

### percentile

|               | `referral`                | `messaging`                      | `econsult`                        |
|:--------------|:--------------------------|:---------------------------------|:--------------------------------- |
| **`100`**     | `428.24 ns` (✅ **1.00x**) | `427.97 ns` (✅ **1.00x faster**) | `424.48 ns` (✅ **1.01x faster**)  |
| **`10000`**   | `72.10 us` (✅ **1.00x**)  | `72.00 us` (✅ **1.00x faster**)  | `72.04 us` (✅ **1.00x faster**)   |
| **`1000000`** | `11.44 ms` (✅ **1.00x**)  | `11.45 ms` (✅ **1.00x slower**)  | `11.40 ms` (✅ **1.00x faster**)   |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

