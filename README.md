# tuid

Sequential UUID generator.

This generator uses a time-based component as the shared prefix.
The prefix is non-monotonic, and wraps around every once in a while to achieve a dense key space.

---

## Table of Contents

* [Motivation](#motivation)
* [Expectations](#expectations)
* [Properties](#properties)
* [Comparison](#comparison)
* [Packages](#packages)
* [Status](#status)
* [License](#license)
* [Acknowledgement](#acknowledgement)

## Motivation
V4 UUIDs - being completely random - have some performance pitfalls when indexed in a database (B-tree) index.
Sequential UUIDs try to avoid this by having some sequential bits as their prefix, while still maintaining a
large number of random bits.

For more details:
- https://www.cybertec-postgresql.com/en/uuid-serial-or-identity-columns-for-postgresql-auto-generated-primary-keys/
- https://www.2ndquadrant.com/en/blog/sequential-uuid-generators/
- https://www.brandur.org/nanoglyphs/026-ids

## Expectations
### You care about
- Generating IDs in application code
- 128-bit UUID compatibility
- Recycling prefixes
  - Denser key space
  - Avoiding lopsided indexes after deletions
  - Harder to guess the order between two IDs
- Ability to use different encoders, RNGs, epochs
- Masking growth insights over time
- Portability across environments

### You don't care about
- Sortability
- Monotonicity
- XKCD 1691

## Properties
### Layout
128-bit UUID compatible
- The prefix bits are based on time
- The remainder bits are randomly generated

### Prefix
The least significant bits of the normalized timestamp
- Controls the wrap around time through overflowing
- Min is 8 bits otherwise it becomes almost totally random
- Max is 48 bits otherwise there isn't enough randomness

### Period
The period (in seconds) where all the generated IDs will share the same prefix.
- Controls how often the prefix is incremented
- Min is 1 seconds
- Max is 4096 seconds

### Defaults
- 16-bit prefix
- 64s period
- 48 days wraparound
- Clock is `CLOCK_MONOTONIC_COARSE` on Linux (or equivalent clocks on other platforms)

## Comparison
[sequential-uuids](https://github.com/tvondra/sequential-uuids)

The main inspiration of this library so pretty similar, with few exceptions:
- Can be generated in application code
- Default period is 64s instead of 60s
  - Slightly better throughput (2%~8% improvement)

[ulid](https://github.com/ulid/spec)
and
[ksuid](https://github.com/segmentio/ksuid)

- Both are meant to be sortable and monotonic, so by design they don't recycle the prefix. However, this may result in a lopsided index when deleting entries
- KSUIDs don't maintain 128-bit UUID compatibility

[cuid](https://github.com/ericelliott/cuid)

- No 128-bit UUID compatibility
- Includes the full timestamp which might leak insights if used for public IDs

## Packages
- [Rust](tuid-rs)
- [Python](tuid-py)

## Status
Hic Sunt Dracones

## License
Mozilla Public License Version 2.0. See the [LICENSE](LICENSE) file for details.

## Acknowledgement
This library is largely inspired by [sequential-uuids](https://github.com/tvondra/sequential-uuids)
