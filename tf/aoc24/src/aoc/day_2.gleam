import aoc/util/input as i
import gleam/int
import gleam/io
import gleam/list
import gleam/result
import gleam/string

pub fn main() {
  let input = i.read(day: 2)
  io.println("Day 2, Part 1: " <> part_one(input))
  io.println("Day 2, Part 2: " <> part_two(input))
}

fn part_one(input: String) {
  input |> count_valid_reports(validate_report)
}

fn part_two(input: String) {
  input |> count_valid_reports(validate_report_tolerant)
}

fn count_valid_reports(input: String, predicate: fn(List(String)) -> Bool) {
  input
  |> string.split("\n")
  |> list.map(fn(line) { line |> string.split(" ") })
  |> list.take_while(fn(x) { x |> list.length() != 1 })
  |> list.map(predicate)
  |> list.count(fn(x) { x == True })
  |> int.to_string()
}

fn validate_report(report: List(String)) {
  let report = list.map(report, fn(n) { n |> int.parse() |> result.unwrap(0) })
  let diffs = calc_differences(report)
  let ascending = list.all(diffs, fn(x) { x < 4 && x > 0 })
  let descending = list.all(diffs, fn(x) { x > -4 && x < 0 })

  ascending || descending
}

fn remove(from list: List(a), at index: Int) -> List(a) {
  { list |> list.take(index) }
  |> list.append(list |> list.drop(index + 1))
}

fn validate_report_tolerant(report: List(String)) {
  // let report = list.map(report, fn(n) { n |> int.parse() |> result.unwrap(0) })
  let range = list.range(0, { report |> list.length } - 1)
  list.map(range, fn(index) { report |> remove(index) |> validate_report })
  |> list.any(fn(res) { res == True })
}

fn calc_differences(report: List(Int)) {
  use acc, cur <- list.fold(report |> list.window_by_2(), [])
  acc |> list.append([cur.0 - cur.1])
}
