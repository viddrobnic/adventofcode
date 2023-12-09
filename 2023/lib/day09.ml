let read_input file_name =
  let file = open_in file_name in
  let lines = In_channel.input_lines file in
  List.map
    (fun line -> String.split_on_char ' ' line |> List.map int_of_string)
    lines

let rec last_element = function
  | [ x ] -> x
  | _ :: xs -> last_element xs
  | [] -> failwith "Empty list"

let print_int_list l =
  List.map string_of_int l |> String.concat " " |> print_string;
  print_newline ()

let rec calculate_differences = function
  | [ x; y ] -> (y - x = 0, [ y - x ])
  | x :: y :: xs ->
      let all_zeros, diffs = calculate_differences (y :: xs) in
      (all_zeros && y - x = 0, (y - x) :: diffs)
  | _ -> failwith "Invalid input"

let rec extrapolate_next_value acc history =
  let all_zeros, diffs = calculate_differences history in
  let next = last_element diffs in
  if all_zeros then next + acc else extrapolate_next_value (next + acc) diffs

let rec extrapolate_previous_value history =
  let all_zeros, diffs = calculate_differences history in
  if all_zeros then List.hd history
  else
    let prev = extrapolate_previous_value diffs in
    List.hd history - prev

let part_one input =
  List.map
    (fun history -> extrapolate_next_value (last_element history) history)
    input
  |> List.fold_left ( + ) 0

let part_two input =
  List.map (fun history -> extrapolate_previous_value history) input
  |> List.fold_left ( + ) 0

let run () =
  let input = read_input "inputs/day_09.txt" in
  let solution_one = part_one input in
  let solution_two = part_two input in
  print_endline ("Part One: " ^ string_of_int solution_one);
  print_endline ("Part Two: " ^ string_of_int solution_two)
