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
