# Token Tide

Your swift navigator token prices.

## Build

```
make build
```

## How to Use Token Tide

1. Simple Query

For a more concise overview, use the `--simple` flag:

```bash
tt query <address/token> --simple
```

Example

```bash
# Input
tt query honey
# Output
Searching HONEY ...
+---------------+----------------------------------------------+
| Property      | Value                                        |
+---------------+----------------------------------------------+
| Pair          | HONEYUSDC                                    |
+---------------+----------------------------------------------+
| Price In USD  | 0.1735                                       |
+---------------+----------------------------------------------+
| Token Address | 4vMsoUT2BWatFweudnQM1xedRLfJgJ7hswhcpz4xgBTy |
+---------------+----------------------------------------------+
```

2. Query Token

To query the price of a specific token description, Use:

```
tt query <address/token>
```

Example:

```
Searching HONEY ...
+---------------+-----------------------------------------------------------------------------+
| Property      | Value                                                                       |
+---------------+-----------------------------------------------------------------------------+
| Pair          | HONEYUSDC                                                                   |
+---------------+-----------------------------------------------------------------------------+
| Price In USD  | 0.1740                                                                      |
+---------------+-----------------------------------------------------------------------------+
| Token Address | 4vMsoUT2BWatFweudnQM1xedRLfJgJ7hswhcpz4xgBTy                                |
+---------------+-----------------------------------------------------------------------------+
| Chain         | solana                                                                      |
+---------------+-----------------------------------------------------------------------------+
| DEX           | raydium                                                                     |
+---------------+-----------------------------------------------------------------------------+
| 24h Volume    | 249.16K (249,161)                                                           |
+---------------+-----------------------------------------------------------------------------+
| FDV           | 1.08B (1,076,745,043)                                                       |
+---------------+-----------------------------------------------------------------------------+
| Liquidity     | $60.48K ($60,480)                                                           |
+---------------+-----------------------------------------------------------------------------+
| Pair Address  | 2RVVkjA9cRHzZgpLiS1s5eRudqF8ZD3kguCGoU1vhjPo                                |
+---------------+-----------------------------------------------------------------------------+
| Link          | https://dexscreener.com/solana/2rvvkja9crhzzgplis1s5erudqf8zd3kgucgou1vhjpo |
+---------------+-----------------------------------------------------------------------------+
```

3. Listing Token Information

To list information about different pairs and exchanges for a specific token address, use:

```bash
tt list <address/token>
```

Example:
```bash
# Input
tt list honey 
# Output
Searching HONEY ...
+--------------+-------------+------------+----------------+---------------+--------------+
| Pair         | Chain       | DEX        | Price In USD   | Token Address | Pair Address |
+--------------+-------------+------------+----------------+---------------+--------------+
| HONEYUSDC    | solana      | raydium    | 0.1718         | 4vMso........ | 2RVVkjA9cR.. |
+--------------+-------------+------------+----------------+---------------+--------------+
...
```



