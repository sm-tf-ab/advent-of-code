import gleam/int
import simplifile

// You cannot commit your inputs to source control! Put them into `./inputs` as
// dayN.txt
pub fn read(day day: Int) -> String {
  case simplifile.read("./inputs/day" <> day |> int.to_string() <> ".txt") {
    Ok(res) -> res
    Error(_) ->
      panic as "Could not load that day. Please make sure that the file is named properly."
  }
}

pub fn read_test(day day: Int) -> String {
  case simplifile.read("./inputs/day" <> day |> int.to_string() <> "test.txt") {
    Ok(res) -> res
    Error(_) ->
      panic as "Could not load that day. Please make sure that the file is named properly."
  }
}
