# Day 5 solution

## Part 1

### The challenge

For this challenge, our puzzle input lists all **seed** kinds that need to be planted and the following maps;

- What **soil** type maps to each **seed** kind.
- What type of **fertiliser** maps to each **soil** kind.
- Types of **water** that map to each **fertiliser**.
- What **light** kind maps to each **water** type.
- What **temperature** maps to each **light** kind.
- Types of **humidity** that map to each **temperature**.
- What **location** maps to each **humidity**.

Gardening related item types are identified via a number, however numbers are re-used by each category (E.g. soil ID 123 doesn't directly relate to fertiliser ID 123).

The example provided is attached to the project [here](src/bin/example.txt).

In the example, we're provided 4 seeds which require planting, `79`, `14`, `55` & `13`.

Underneath we can then see each other "mapper" sharing the same structure as shown below:

	seed-to-soil map:
	50 98 2
	52 50 48

These inform us as to how we should convert numbers from the **source** category into numbers in **destination** category.

In order, the numbers are:
1. The start of the destination range (E.g. Soil).
2. The start of the source range (E.g. Seed).
3. The range applied to each of the ranges defined prior.

Referring to the previous seed-to-soil example;

| Line 1 (has range of 2) ||
| Source (Seed) | Destination (Soil) |
|:--|:--|
| 98 | 50 |
| 99 | 51 |

With this understanding of the first line of seed-to-soil mapper, we now know that **seed** number `99` maps to **soil** number `51`.

| Line 2 (has range of 48) ||
| Source (Seed) | Destination (Soil) |
|:--|:--|
| 50 | 52 |
| 51 | 53 |
| ... | ... |
| 96 | 98 |
| 97 | 99 |

With this understanding of the first line of seed-to-soil mapper, we now know that **seed** number `53` maps to **soil** number `55`.

Any source numbers that are not mapped, identically match their destination numbers (E.g. Given seed ID 10 has not been mapped, we now know it maps to soil ID 10).

We can then combine these two ranges to come up with a mapper as shown below:

| Seed to Soil Mapper ||
| Source (Seed) | Destination (Soil) |
|:--|:--|
| 0 | 0 |
| 1 | 1 |
| ... | ... |
| 48 | 48 |
| 49 | 49 |
| 50 | 52 |
| 51 | 53 |
| ... | ... |
| 96 | 98 |
| 97 | 99 |

We can then decipher that:
- Seed ID `79` maps to soil ID `81`.
- Seed ID `14` maps to soil ID `14`.
- Seed ID `55` maps to soil ID `57`.
- Seed ID `13` maps to soil ID `13`.

For each seed, we'll need to journey through each mapper in order to retrieve a location ID for each seed.

We then need to return the lowest location ID, which in the example is `35`.

### The thought process

For each mapper, if the source ID provided falls within a defined **source range**, we need to calculate the difference between the source start and the destination start. We'll then need to sum the difference against the source ID provided. Otherwise we return the source ID itself as a source range has not been defined. 

Applying this thought process to the example above, let's look at the mapper example again:
	
	seed-to-soil map:
	50 98 2
	52 50 48

Here we have two source ranges:
- `98-99`, with a difference of `-48` after taking 98 (source start) away from 50 (source start).
- `50-98`, with a difference of `2` after taking 50 (source start) away from 52 (destination start).

With these source ranges defined, we can then access whether a given source ID falls within one of these ranges. If the source ID falls within a range, return the sum of the source ID against the difference of the range, else return the source ID itself.

Taking `56` as an example, because it falls within the `50-98` defined source range, we'll return `56 + (52 - 50)` which is `58`; 

Whereas with `99`, because it falls within the `98-99` defined source range, we'll return `99 + (50 - 98)` which is `51`.