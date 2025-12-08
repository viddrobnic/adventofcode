open Aoc

let () =
  let args = Sys.argv in
  let day =
    if Array.length args < 2 then failwith "Missing day parameter" else args.(1)
  in

  match day with
  | "1" -> Day01.run ()
  | "2" -> Day02.run ()
  | "3" -> Day03.run ()
  | "4" -> Day04.run ()
  | "5" -> Day05.run ()
  | "6" -> Day06.run ()
  | "7" -> Day07.run ()
  | "8" -> Day08.run ()
  | _ -> failwith "Invalid day"
