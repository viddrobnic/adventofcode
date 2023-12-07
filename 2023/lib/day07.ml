let count_cards hand =
  List.fold_left
    (fun counts card ->
      let exists = List.find_opt (fun (c, _) -> c == card) counts in
      match exists with
      | None -> (card, 1) :: counts
      | Some (c, count) ->
          (c, count + 1) :: List.filter (fun (c, _) -> c != card) counts)
    [] hand

let sorted_hand_to_int = function
  | [ 1; 1; 1; 1; 1 ] -> 0
  | [ 2; 1; 1; 1 ] -> 1
  | [ 2; 2; 1 ] -> 2
  | [ 3; 1; 1 ] -> 3
  | [ 3; 2 ] -> 4
  | [ 4; 1 ] -> 5
  | [ 5 ] -> 6
  | _ -> failwith "Invalid hand"

let int_of_hand_part_one hand =
  let counts = count_cards hand |> List.map (fun (_, count) -> count) in
  let sorted = List.sort (fun a b -> compare b a) counts in
  sorted_hand_to_int sorted

let int_of_hand_part_two hand =
  let counts = count_cards hand in
  let jack =
    match List.find_opt (fun (c, _) -> c == 'J') counts with
    | None -> 0
    | Some (_, count) -> count
  in
  let counts =
    List.filter_map
      (fun (c, count) -> if c != 'J' then Some count else None)
      counts
  in
  let sorted = List.sort (fun a b -> compare b a) counts in
  let sorted =
    match sorted with [] -> [ jack ] | x :: xs -> (x + jack) :: xs
  in

  try sorted_hand_to_int sorted
  with _ ->
    failwith ("Invalid hand: " ^ String.concat "" (List.map Char.escaped hand))

let int_of_card_part_one card =
  match card with
  | '2' -> 0
  | '3' -> 1
  | '4' -> 2
  | '5' -> 3
  | '6' -> 4
  | '7' -> 5
  | '8' -> 6
  | '9' -> 7
  | 'T' -> 8
  | 'J' -> 9
  | 'Q' -> 10
  | 'K' -> 11
  | 'A' -> 12
  | _ -> failwith ("Invalid card: " ^ Char.escaped card)

let int_of_card_part_two card =
  match card with
  | 'J' -> 0
  | '2' -> 1
  | '3' -> 2
  | '4' -> 3
  | '5' -> 4
  | '6' -> 5
  | '7' -> 6
  | '8' -> 7
  | '9' -> 8
  | 'T' -> 9
  | 'Q' -> 10
  | 'K' -> 11
  | 'A' -> 12
  | _ -> failwith ("Invalid card: " ^ Char.escaped card)

let compare_hands f_map_card hand1 hand_type1 hand2 hand_type2 =
  if hand_type1 > hand_type2 then 1
  else if hand_type1 < hand_type2 then -1
  else
    let rec compare_cards hand1 hand2 =
      match (hand1, hand2) with
      | [], [] -> 0
      | h1 :: t1, h2 :: t2 ->
          let card1 = f_map_card h1 in
          let card2 = f_map_card h2 in

          if card1 > card2 then 1
          else if card1 < card2 then -1
          else compare_cards t1 t2
      | _ -> failwith "Invalid hands"
    in
    compare_cards hand1 hand2

let read_input file_name =
  let ic = open_in file_name in
  let lines = In_channel.input_lines ic in
  List.map
    (fun line ->
      let parts = String.split_on_char ' ' line in
      match parts with
      | [ cards; bid ] ->
          let hand = String.to_seq cards |> List.of_seq in
          (hand, int_of_string bid)
      | _ -> failwith "Invalid input")
    lines

let solve f_map_hand f_map_card input =
  let input = List.map (fun (h, b) -> (h, f_map_hand h, b)) input in
  List.sort
    (fun (h1, ht1, _) (h2, ht2, _) -> compare_hands f_map_card h1 ht1 h2 ht2)
    input
  |> List.fold_left
       (fun (total, index) (_, _, bid) -> (total + (bid * index), index + 1))
       (0, 1)
  |> fst

let part_one input = solve int_of_hand_part_one int_of_card_part_one input
let part_two input = solve int_of_hand_part_two int_of_card_part_two input

let run () =
  let input = read_input "inputs/day_07.txt" in
  let solution_one = part_one input in
  let solution_two = part_two input in
  print_endline ("Part One: " ^ string_of_int solution_one);
  print_endline ("Part Two: " ^ string_of_int solution_two)
