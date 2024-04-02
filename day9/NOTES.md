# Notes

## Data structures

### Report

- Type: Struct
- Properties:
  - Areas: Area list

### Area

- Type: Struct
- Properties:
  - History: Value list

### Value

- Type: Struct wrapping an i32
  - Highest recorded value: 21793992
  - Lowest recorded value: -1698921

## Solution

1. Parse into Report data structure

a. Parse values
b. Parse areas

2. Get next value prediction

a. Create new "difference" vectors until we get a vector containing all zeros.
b. Add zero to last vector of zeros.
c. Starting from one before last vector, calculating next prediction.
d. Return last value of original vector.

3. Calculate the total sum of the next value predictions

a. Sum all last values returned from original vector.
