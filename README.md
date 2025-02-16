# Test concurrent code in Rust with Loom

This material is part of my talk at the Rust Tbilisi kickoff [meetup](https://www.meetup.com/tbilisi-rustaceans/events/297231022) (18.11.2023) .

## Description of the Problem

Five philosophers dine together at the same table. Each philosopher has their own place at the table, and there is a fork between each plate. The dish served is a kind of spaghetti that must be eaten with two forks.
Each philosopher can only alternate between thinking and eating. Moreover, a philosopher can only eat their spaghetti when they have both a left and a right fork. Thus, two forks will only be available when their two nearest neighbors are thinking rather than eating. After an individual philosopher finishes eating, they put down both forks.

![Description](./assets/description.webp)
The challenge of this problem arises from the need to manage concurrent access to the limited resources (the forks) while avoiding deadlock (where no progress is made because all philosophers are waiting indefinitely to acquire forks) and starvation (where a philosopher never gets to eat because others always take precedence).

## Description of the Solution

This implementation of the Dining Philosophers problem provides two solutions:

1. **Naive Solution**:
   - In the naive solution, each philosopher tries to pick up their left fork and then their right fork in sequence.
   - This approach can lead to a deadlock situation where each philosopher holds one fork and waits indefinitely for the other fork to become available.

2. **Deadlock-Free Solution**:
   - The deadlock-free solution introduces a break in symmetry. Specifically, one philosopher (the last in the seating arrangement) switches the order in which they pick up the forks. Instead of picking up the left fork first, then the right fork, they take the right fork first and then the left fork.
   - By doing this, the circular wait condition necessary for deadlock is prevented, allowing the program to run indefinitely without freezing.

Both solutions are tested using [Loom](https://github.com/tokio-rs/loom), a tool in Rust to check for concurrency issues. `loom` allows you to test all possible interleavings of threads, ensuring the robustness of concurrent algorithms.

## How to Run the Tests with Loom Enabled

To run the tests using `loom`, ensuring thread safety and the absence of deadlocks:

**Run Tests**:
You can run the tests with `loom` by enabling the `loom` configuration:
 - For triggering a bug:
```bash
LOOM_LOG=info LOOM_MAX_PREEMPTIONS=2 RUSTFLAGS="--cfg loom" cargo test -r -- --exact trigger_deadlock_with_loom --nocapture
```
- For running without a bug:
```bash
LOOM_LOG=info LOOM_MAX_PREEMPTIONS=2 RUSTFLAGS="--cfg loom" cargo test -r -- --exact run_correct_solution_with_loom --nocapture
```

Notes:

When running with `loom`, the code is tested with reduced iterations (10 instead of 100) because `loom` performs an exhaustive exploration of all possible thread interleavings. This reduction is necessary since the exhaustive checking provided by `loom` can become computationally expensive with higher iteration counts.

Additionally, the number of philosophers is reduced to 3 instead of 5 to further simplify the concurrency scenario, thereby limiting the state space and ensuring that `loom` can effectively analyze all interleavings. `LOOM_MAX_PREEMPTIONS=2` is also used to cap the number of preemptions, which controls the number of interleavings and makes the thorough testing by `loom` practical and efficient.
