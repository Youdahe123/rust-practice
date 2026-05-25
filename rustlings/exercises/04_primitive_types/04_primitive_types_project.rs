// ============================================================
//  PRIMITIVE TYPES MINI PROJECT: Sensor Dashboard
//  Topics: arrays, slices, tuples, u8 / f64 / bool / char
//  Difficulty: Medium-Hard
// ============================================================
//
//  You are building a mini sensor dashboard for 12 environmental
//  readings (0–100 scale). You'll need to:
//    1. Declare a typed array of readings
//    2. Implement three functions that work with slices + tuples
//    3. Destructure the returned tuples in main
//    4. Slice the array to analyze just the middle section
//
//  DO NOT change any println! calls or function signatures.
//  All TODOs are yours to fill in.
// ============================================================

const SENSOR_COUNT: usize = 12;

fn main() {
    // ── TODO 1 ───────────────────────────────────────────────
    // Declare a [u8; SENSOR_COUNT] array called `readings` with
    // EXACTLY these values (in order):
    //   34, 87, 22, 95, 61, 43, 78, 10, 55, 89, 47, 66
    //
    // let readings: [u8; SENSOR_COUNT] = ???;
    todo!("TODO 1: declare the readings array");

    // ── TODO 2 ───────────────────────────────────────────────
    // Call compute_stats() passing a slice of the FULL array.
    // Destructure the returned tuple into three variables:
    //   min  (u8)
    //   max  (u8)
    //   avg  (f64)
    //
    // let (???, ???, ???) = compute_stats(???);
    todo!("TODO 2: call compute_stats and destructure the result");

    println!("Full array  → Min: {min}, Max: {max}, Avg: {avg:.2}");

    // ── TODO 3 ───────────────────────────────────────────────
    // Slice `readings` to get ONLY indices 3..9 (the middle 6).
    // Then call compute_stats on that slice and destructure again
    // into mid_min, mid_max, mid_avg.
    //
    // let mid_slice = ???;
    // let (???, ???, ???) = compute_stats(mid_slice);
    todo!("TODO 3: slice the middle 6 and compute stats on them");

    println!("Middle 6    → Min: {mid_min}, Max: {mid_max}, Avg: {mid_avg:.2}");

    // ── TODO 4 ───────────────────────────────────────────────
    // Call sensor_status(avg) and destructure its return value into:
    //   label       (char)
    //   is_critical (bool)
    //
    // let (???, ???) = sensor_status(avg);
    todo!("TODO 4: call sensor_status and destructure");

    println!("Status      → Label: '{label}', Critical: {is_critical}");

    // ── TODO 5 ───────────────────────────────────────────────
    // Call count_threshold passing the full slice and `avg`.
    // Destructure into:
    //   below (u32)
    //   above (u32)
    //
    // let (???, ???) = count_threshold(???, avg);
    todo!("TODO 5: call count_threshold and destructure");

    println!("Threshold   → {below} below avg, {above} at-or-above avg");

    // ── BONUS (optional) ─────────────────────────────────────
    // Access the SECOND element of `readings` using a tuple-style
    // index (the way you learned in primitive_types6) … wait, arrays
    // don't work like that. Use a regular index instead and print it.
    // Then try: what happens if you index out of bounds at compile time
    // with a constant? (comment it out after you see the error)
    println!("Second reading: {}", readings[1]);
}

// ─────────────────────────────────────────────────────────────
//  IMPLEMENT THE THREE FUNCTIONS BELOW
// ─────────────────────────────────────────────────────────────

/// Returns (min, max, mean) for a slice of u8 sensor readings.
///
/// Requirements:
///  - Walk the slice with a for loop (no iterators / .iter().min() etc.)
///  - min and max stay as u8
///  - mean must be computed as f64 (cast each u8 with `as f64`)
///  - Assume the slice is never empty (no need to handle that case)
fn compute_stats(data: &[u8]) -> (u8, u8, f64) {
    // Hints:
    //   • Start min at u8::MAX and max at u8::MIN
    //   • Accumulate a f64 sum as you loop
    //   • Return a tuple literal: (min, max, sum / data.len() as f64)
    todo!("implement compute_stats")
}

/// Returns (status_label, is_critical) based on the average reading.
///
///  avg >= 60.0  →  label = 'G' (Good),     is_critical = false
///  avg >= 35.0  →  label = 'W' (Warning),  is_critical = false
///  avg <  35.0  →  label = 'C' (Critical), is_critical = true
fn sensor_status(avg: f64) -> (char, bool) {
    // Use an if / else if / else chain.
    // Return a tuple literal of (char, bool).
    todo!("implement sensor_status")
}

/// Returns (count_below, count_at_or_above) relative to threshold.
///
/// For each element in `data`:
///   if (element as f64) < threshold  →  count_below  += 1
///   otherwise                        →  count_above  += 1
fn count_threshold(data: &[u8], threshold: f64) -> (u32, u32) {
    // Declare two u32 counters, loop through the slice, cast to f64,
    // compare, increment the right counter, and return the tuple.
    todo!("implement count_threshold")
}
