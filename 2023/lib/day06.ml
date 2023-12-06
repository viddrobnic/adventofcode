let read_input file_name =
  let ic = open_in file_name in
  let lines = In_channel.input_lines ic in
  List.map
    (fun line ->
      match String.split_on_char ':' line with
      | [ _; numbers ] -> String.trim numbers
      | _ -> failwith "Invalid input")
    lines

let nr_wins time distance =
  let d =
    Float.sqrt
      (Float.pow (float_of_int time) 2.0 -. (4.0 *. float_of_int distance))
  in
  let timef = float_of_int time in
  let x1 = int_of_float (Float.ceil ((timef -. d) /. 2.0)) in
  let x2 = int_of_float (Float.floor ((timef +. d) /. 2.0)) in
  let x1 = if distance == x1 * (time - x1) then x1 + 1 else x1 in
  let x2 = if distance == x2 * (time - x2) then x2 - 1 else x2 in
  x2 - x1 + 1

let part_one input =
  let numbers =
    List.map
      (fun line -> Str.split (Str.regexp " +") line |> List.map int_of_string)
      input
  in
  let input =
    match numbers with
    | [ times; distances ] -> List.combine times distances
    | _ -> failwith "Invalid input"
  in

  List.fold_left
    (fun acc (time, distance) ->
      let wins = nr_wins time distance in
      acc * wins)
    1 input

let part_two input =
  let input =
    List.map
      (fun line ->
        Str.global_replace (Str.regexp " +") "" line |> int_of_string)
      input
  in
  let time, distance =
    match input with
    | [ time; distance ] -> (time, distance)
    | _ -> failwith "Invalid input"
  in
  nr_wins time distance

let run () =
  let input = read_input "inputs/day_06.txt" in
  let result_one = part_one input in
  let result_two = part_two input in
  print_endline ("Part one: " ^ string_of_int result_one);
  print_endline ("Part two: " ^ string_of_int result_two)
