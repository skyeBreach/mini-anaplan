# Anaplan Calculation Engines

Anaplan offers two different calculation engines,

- Classic
- Polaris

## Density

The calculation engine is set at the workspace level, so the composition of all data in a workspace must be
taken into account when considering memory efficiency.

Each engine uses memory in different ways:

- Classic engine uses 8 bytes per cell, even if it is empty.
- Polaris only uses 24 bytes per non-empty cell

The density of the data, defines which engine is more efficient, such that A workspace is considered to be:

- **Dense** when more than 33% of cells contain data.
- **Sparse** when less than 33% of cells contain data

The Polaris engine is designed to deal with **Sparse** data sets, where as the classic is more memory efficient
for **Dense** data.
