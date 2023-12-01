let line_to_number line =
  let pair =
    String.fold_left
      (fun acc c ->
        match (acc, c) with
        | (None, _), '0' .. '9' -> (Some c, Some c)
        | (Some a, _), '0' .. '9' -> (Some a, Some c)
        | _ -> acc)
      (None, None) line
  in
  match pair with
  | Some a, Some b -> int_of_string (Char.escaped a ^ Char.escaped b)
  | _ -> failwith ("Invalid input: " ^ line)

let read_input () =
  let ic = open_in "inputs/day_01.txt" in
  In_channel.input_lines ic

let part_one input =
  let num_lines = List.map line_to_number input in
  List.fold_left ( + ) 0 num_lines

let part_two input =
  let cleaned_lines =
    List.map
      (fun line ->
        line
        |> Str.global_replace (Str.regexp "one") "one1one"
        |> Str.global_replace (Str.regexp "two") "two2two"
        |> Str.global_replace (Str.regexp "three") "three3three"
        |> Str.global_replace (Str.regexp "four") "four4four"
        |> Str.global_replace (Str.regexp "five") "five5five"
        |> Str.global_replace (Str.regexp "six") "six6six"
        |> Str.global_replace (Str.regexp "seven") "seven7seven"
        |> Str.global_replace (Str.regexp "eight") "eight8eight"
        |> Str.global_replace (Str.regexp "nine") "nine9nine")
      input
  in
  let num_lines = List.map line_to_number cleaned_lines in
  List.fold_left ( + ) 0 num_lines

let run () =
  let input = read_input () in
  let part_one = part_one input in
  let part_two = part_two input in
  print_endline ("Part One: " ^ string_of_int part_one);
  print_endline ("Part Two: " ^ string_of_int part_two)
