open Aoc

let () =
  let args = Sys.argv in
  let day = args.(1) in
  match day with "1" -> Day01.run () | _ -> failwith "Invalid day"
