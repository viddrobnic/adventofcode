open Aoc

let () =
  let args = Sys.argv in
  let day = args.(1) in
  match day with
  | "1" -> Day01.run ()
  | "2" -> Day02.run ()
  | "3" -> Day03.run ()
  | "4" -> Day04.run ()
  | "5" -> Day05.run ()
  | "6" -> Day06.run ()
  | "7" -> Day07.run ()
  | "8" -> Day08.run ()
  | "9" -> Day09.run ()
  | "10" -> Day10.run ()
  | "11" -> Day11.run ()
  | "12" -> Day12.run ()
  | "13" -> Day13.run ()
  | "14" -> Day14.run ()
  | "15" -> Day15.run ()
  | "16" -> Day16.run ()
  | _ -> failwith "Invalid day"
