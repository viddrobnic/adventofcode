type spring = Damaged | Operational | Unknown

let print_springs springs =
  List.map
    (fun s ->
      match s with Damaged -> "#" | Operational -> "." | Unknown -> "?")
    springs
  |> String.concat "" |> print_endline

let read_input filename =
  let file = open_in filename in
  let lines = In_channel.input_lines file in
  List.map
    (fun line ->
      match String.split_on_char ' ' line with
      | [ springs; groups ] ->
          let springs =
            String.to_seq springs
            |> Seq.map (fun c ->
                   match c with
                   | '.' -> Operational
                   | '#' -> Damaged
                   | _ -> Unknown)
            |> List.of_seq
          in

          let groups =
            String.split_on_char ',' groups |> List.map int_of_string
          in

          (springs, groups)
      | _ -> failwith "Invalid input")
    lines

let rec possible_arrangements memo springs current_group groups =
  try Hashtbl.find memo (springs, current_group, groups)
  with Not_found ->
    let res =
      match (springs, current_group, groups) with
      | [], None, [] -> 1
      | [], _, _ -> 0
      | Operational :: sprgs, Some 0, grps ->
          possible_arrangements memo sprgs None grps
      | Operational :: _, Some _, _ -> 0
      | Operational :: sprgs, None, grps ->
          possible_arrangements memo sprgs None grps
      | Damaged :: _, Some 0, _ -> 0
      | Damaged :: sprgs, Some g, grps ->
          possible_arrangements memo sprgs (Some (g - 1)) grps
      | Damaged :: sprgs, None, g :: grps ->
          possible_arrangements memo sprgs (Some (g - 1)) grps
      | Damaged :: _, None, [] -> 0
      | Unknown :: sprgs, Some 0, grps ->
          possible_arrangements memo sprgs None grps
      | Unknown :: sprgs, Some g, grps ->
          possible_arrangements memo sprgs (Some (g - 1)) grps
      | Unknown :: sprgs, None, [] -> possible_arrangements memo sprgs None []
      | Unknown :: sprgs, None, g :: grps ->
          possible_arrangements memo sprgs (Some (g - 1)) grps
          + possible_arrangements memo sprgs None (g :: grps)
    in
    Hashtbl.add memo (springs, current_group, groups) res;
    res

let part_one input =
  List.fold_left
    (fun acc (springs, groups) ->
      acc
      + possible_arrangements (Hashtbl.create 512)
          (springs @ [ Operational ])
          None groups)
    0 input

let part_two input =
  let expand (springs, groups) =
    let springs =
      springs @ (Unknown :: springs) @ (Unknown :: springs)
      @ (Unknown :: springs) @ (Unknown :: springs)
    in
    let groups = groups @ groups @ groups @ groups @ groups in
    (springs, groups)
  in

  List.map expand input |> part_one

let run () =
  let input = read_input "inputs/day_12.txt" in
  let solution_one = part_one input in
  let solution_two = part_two input in
  Printf.printf "Part one: %d\n" solution_one;
  Printf.printf "Part two: %d\n" solution_two
