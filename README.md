# 1 Billion Rows Challenge

Test Measurement size is set to `1_00_000_000`

## Naive Implementation

1. Read all the contents of file at once.
2. Split by '\n' and then '\s' to form records, use `parse` to make `f32`
3. Store records in a vec
4. Create btree map out of the vec
5. Create individual city record from btree map and sort the recordings as well
6. Iterate over city records and call min(O(1)), max (O(1)) and average (takes O(n) time).

>Tried using buf reader but results are still the same as we are reading all the data and also doing so many copies.

Time: `25s`

## Inplace Update

1. Read each record one by one
2. Update the entries in a BTree
3. We keep sum and count additional entries and mean is a method call
3. Iterate over values in BTree and print the most upto date entries

Time: `20s`

>Using Hashmap instead of BTree reduces time to 15 seconds.

## Split Method

The only change required was using `split_once` instead of `splitn` which prevents clone of the whole split record and then an additional `to_string` for city name.

Now we just need one `to_string` for city name.

Time: `11s`

## New Packages

- `fast-float` didn't help much.
- `rustc-hash` has `FxHashMap` which is a very fast implementation.
- `ahash` by changing the hasher to a non cryptographic on from this crate in 1 billion case we say better results.
- `rayon` for collecting string parallely but that is not very cpu intensive and doesn't reduce the time much.

Overall we got a reduction of `2s` so final is `9s` now.
