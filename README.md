# calculator
## demo
<img src="https://github.com/user-attachments/assets/d37e755b-7c84-4086-845e-89df1de3b741" width="70%">

## usage
```
Values types:
    Numbers(i64 and f64, examples: 1, 2.0, -3.3333)
    List(it accepts different types in it, examples: [1, 2, 3], [], [[1], 0.23, [1, 0, -2]])
Arithmetic options:
    Addition: a + b
    Subtraction: a - b
    Multiplication: a * b
    Division: a / b
    * Notice that it's legal to multiplication or division a number with a list. For example, [1, 2, 3] * 3 = [3, 6, 9]
Supports the following constants:
    pi
    e
Offer the following predefined function:
    sin(x), cos(x), tan(x)
    asin(x), acos(x), atan(x)
    ln(x), log10(x), log2(2), log(x, base)
    round(x), floor(x), ceil(x)
    sqrt(x), exp(x), pow(x, e), abs(x)
    min(x, y), max(x, y)
    min(list), max(list)
    rand(), rand(stop), rand(start, stop): random float (default range is 0.0 to 10.0)
    map(f, list): applies f to each element of list
    sort(list)
    length(list)
Also you can define function by yourself:
    For example, f(x) = x+1.
    Then, when you input f(5), you'll get 6.
* Note that if you have a illegial operation, the program is not suppose to stop. It'll tell you the error and you can continue input it.
```
