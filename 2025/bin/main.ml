open Aoc

let () =
  let args = Sys.argv in
  let day =
    if Array.length args < 2 then failwith "Missing day parameter" else args.(1)
  in

  match day with "1" -> Day01.run () | _ -> failwith "Invalid day"
