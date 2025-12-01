type direction = Left | Right

let parse_line line =
  let dir = line.[0] in
  let steps = String.sub line 1 (String.length line - 1) |> int_of_string in
  match dir with
  | 'L' -> (Left, steps)
  | 'R' -> (Right, steps)
  | _ -> failwith "Invalid direction"

let read_input () =
  let ic = open_in "inputs/day01.txt" in
  let lines = In_channel.input_lines ic in
  List.map parse_line lines

let pos_mod a n =
  let r = a mod n in
  if r < 0 then r + n else r

let part_one input =
  let res, _ =
    List.fold_left
      (fun (res, pos) (dir, steps) ->
        let new_pos = pos + match dir with Left -> -steps | Right -> steps in
        let new_pos = pos_mod new_pos 100 in
        if new_pos = 0 then (res + 1, new_pos) else (res, new_pos))
      (0, 50) input
  in
  res

let part_two input =
  let res, _ =
    List.fold_left
      (fun (res, pos) (dir, steps) ->
        let new_pos = pos + match dir with Left -> -steps | Right -> steps in
        let clicks = abs (new_pos / 100) in
        let clicks = clicks + if pos > 0 && new_pos < 0 then 1 else 0 in
        let clicks = clicks + if new_pos = 0 then 1 else 0 in
        Printf.printf "%d\t->\t%d:\t%d\n" pos new_pos clicks;
        let new_pos = pos_mod new_pos 100 in
        (res + clicks, new_pos))
      (0, 50) input
  in
  res

let run () =
  let input = read_input () in
  let p_one = part_one input in
  let p_two = part_two input in
  Printf.printf "Part one: %d\n" p_one;
  Printf.printf "Part two: %d\n" p_two
