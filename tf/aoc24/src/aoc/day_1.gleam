import aoc/util/input as i
import gleam/int
import gleam/io
import gleam/list
import gleam/result
import gleam/string

// NOTE: You can run each file as a module if it includes the `main()` function.
// You can do so by running `gleam run -m aoc/day_1`, for example.
//
// So, we need a function to parse the input into a list of tuples. 
// For this challenege, we need to sort them first, then transform into tuples.
// They also have to be output as `Int` so we can do the math with them.
pub fn main() {
  let input = i.read(day: 1)
  io.println("Day 1, Part 1: " <> part_one(input))
  io.println("Day 1, Part 2: " <> part_two(input))
}

//  PART ONE 

fn part_one(input: String) -> String {
  let input = input |> prepare_input()
  let sorted_pairs = input |> sort_into_pairs()

  sorted_pairs
  |> calculate_distances()
  |> int.sum()
  |> int.to_string()
}

fn prepare_input(input: String) -> #(List(Int), List(Int)) {
  // To make the input predictable, we can convert newlines to "  " such that
  // all numbers are separated with a "  ". Then we can just say that even
  // indexes are for left and odd is for right.
  let del = "   "
  let sp = input |> string.replace("\n", del) |> string.split(del)
  sp |> do_parse_lists(#([], []))
}

fn sort_into_pairs(lists: #(List(Int), List(Int))) -> List(#(Int, Int)) {
  let sorted_left = lists.0 |> list.sort(by: int.compare)
  let sorted_right = lists.1 |> list.sort(by: int.compare)

  // This takes [a, b] and [c, d] and makes [#(a, c), #(b, d)]
  list.zip(sorted_left, sorted_right)
}

// Recursive function which takes input and accumulator.
fn do_parse_lists(
  input: List(String),
  acc: #(List(Int), List(Int)),
) -> #(List(Int), List(Int)) {
  case input {
    // Capture first and second in the list and relegate to correct side.
    [left, right, ..rest] -> {
      let left = left |> int.parse |> result.unwrap(0)
      let right = right |> int.parse |> result.unwrap(0)

      // You access tuples using index. E.g. #(a, b).1 => b
      do_parse_lists(rest, #(
        acc.0 |> list.append([left]),
        acc.1 |> list.append([right]),
      ))
    }
    // Complete case -- input is empty
    [] | [""] -> acc
    // Catch-all case.
    [_] -> panic as "Input malformed. Uneven number."
  }
}

fn calculate_distances(pairs: List(#(Int, Int))) -> List(Int) {
  pairs
  |> list.map(fn(pair) { { pair.0 - pair.1 } |> int.absolute_value() })
}

//  PART TWO 

fn part_two(input: String) -> String {
  let #(left, right) = prepare_input(input)
  count_scores(left, right) |> int.sum() |> int.to_string()
}

fn count_scores(left: List(Int), right: List(Int)) -> List(Int) {
  // I made this as difficult to understand as possible.
  // Once you understand it though, it may be quite sweet :)
  use n, count <- list.map2(left, {
    use l <- list.map(left)
    use r <- list.count(right)
    l == r
  })
  n * count
}
