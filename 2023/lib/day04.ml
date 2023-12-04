let read_input file_name =
  let ic = open_in file_name in
  let lines = In_channel.input_lines ic in
  List.map
    (fun lin ->
      let tickets = List.nth (String.split_on_char ':' lin) 1 in
      let parts = String.split_on_char '|' tickets in

      let winning_numbers =
        List.nth parts 0 |> String.trim
        |> Str.split (Str.regexp " +")
        |> List.map int_of_string
      in
      let my_tickets =
        List.nth parts 1 |> String.trim
        |> Str.split (Str.regexp " +")
        |> List.map int_of_string
      in
      (winning_numbers, my_tickets))
    lines

let get_matching_numbers input =
  List.map
    (fun (winning_number, my_numbers) ->
      List.fold_left
        (fun acc number ->
          if List.mem number winning_number then acc + 1 else acc)
        0 my_numbers)
    input

let part_one input =
  let rec score acc = function
    | 0 -> 0
    | 1 -> acc
    | n -> score (acc * 2) (n - 1)
  in
  let matching_numbers = get_matching_numbers input in
  List.fold_left (fun acc number -> acc + score 1 number) 0 matching_numbers

let part_two input =
  let matching_numbers = get_matching_numbers input in
  let nr_cards = Array.init (List.length matching_numbers) (fun _ -> 1) in

  List.iteri
    (fun i number ->
      List.init number (fun j -> i + j + 1) (* Indices of cards which we won *)
      |> List.iter (fun j -> nr_cards.(j) <- nr_cards.(j) + nr_cards.(i)))
    matching_numbers;

  Array.fold_left ( + ) 0 nr_cards

let run () =
  let input = read_input "inputs/day_04.txt" in
  let result_one = part_one input in
  let result_two = part_two input in
  print_endline ("Part one: " ^ string_of_int result_one);
  print_endline ("Part two: " ^ string_of_int result_two)
