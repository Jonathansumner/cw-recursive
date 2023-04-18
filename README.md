# Recursive Contract
This contract is used exclusively for testing purposes at present.

- When instantiated this contract will recursively instantiate itself to a specified depth.
- This functionality is necessary to test the handling of recursive contracts within the [ledger-subquery](www.github.com/fetchai/ledger-subquery.git) indexer.


#### Example Instantiation Arguments
```
// This example would recursively instantiate a stored contract three times
{
  "code_id": code_id_of_contract, // The code_id of the stored contract to recursively instantiate
  "depth": 3 // The number of times to recurse
}
```
