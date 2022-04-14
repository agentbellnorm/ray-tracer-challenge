# Performance
| chapter | pixels | total ms | ms/px       | comment                                                                                                            |
|---------|--------|----------|-------------|--------------------------------------------------------------------------------------------------------------------|
| 7       | 5000   | 4441     | 0.8862      |                                                                                                                    |
| 7       | 720000 | 618091   | 0.8584597   |                                                                                                                    |
| 8       | 45000  | 67437    | 1.4986      | add shadow tracing                                                                                                 |
| 8       | 45000  | 64976    | 1.44391111  | switch to f64                                                                                                      |
| 8       | 720000 | 1082876  | 1.503994444 |                                                                                                                    |
| 9       | 960000 | 773568   | 0.8058      | three spheres, one plane (fast i think). refactoring, but shouldn't affect performance                             |
| 10      | 720000 | 525486   | 0.729841    | switched from vec to array for matrices. running from main instead of test. add patterns. three spheres two planes |


