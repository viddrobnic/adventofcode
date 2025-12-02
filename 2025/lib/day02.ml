let read_input () =
  let ic = open_in "inputs/day02.txt" in
  let line = In_channel.input_line ic |> Option.get in
  let intervals = String.split_on_char ',' line in
  List.map
    (fun interval ->
      let parts = String.split_on_char '-' interval in
      match parts with
      | [ left; right ] -> (int_of_string left, int_of_string right)
      | _ -> failwith "invalid interval")
    intervals

let reg_one = Str.regexp {|^\(.+\)\1$|}
let reg_two = Str.regexp {|^\(.+\)\1+$|}
let is_invalid reg n = Str.string_match reg (string_of_int n) 0

let sum_invalid reg (left, right) =
  let rec aux left right acc =
    if left > right then acc
    else
      let acc = if is_invalid reg left then acc + left else acc in
      aux (left + 1) right acc
  in
  aux left right 0

let solve reg =
  List.fold_left (fun acc interval -> acc + sum_invalid reg interval) 0

let part_one = solve reg_one
let part_two = solve reg_two

let run () =
  let input = read_input () in
  let p_one = part_one input in
  let p_two = part_two input in
  Printf.printf "Part one: %d\n" p_one;
  Printf.printf "Part two: %d\n" p_two
