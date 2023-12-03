open Aoc

let () =
  let args = Sys.argv in
  let day = args.(1) in
  match day with
  | "1" -> Day01.run ()
  | "2" -> Day02.run ()
  | "3" -> Day03.run ()
  | _ -> failwith "Invalid day"
