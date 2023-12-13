let read_input filename =
  let file = open_in filename in
  let lines = In_channel.input_lines file in

  let res, last =
    List.fold_left
      (fun (res, acc) line ->
        match line with
        | "" -> (List.rev acc :: res, [])
        | _ -> (res, (String.to_seq line |> List.of_seq) :: acc))
      ([], []) lines
  in

  List.rev (List.rev last :: res)

let rec transpose = function
  | [] -> []
  | [] :: _ -> []
  | m -> List.map List.hd m :: transpose (List.map List.tl m)

let horizontal_reflections map =
  let rec check_reflected initial_call up down =
    match (up, down) with
    | [], _ -> not initial_call
    | _, [] -> not initial_call
    | u :: us, d :: ds -> u = d && check_reflected false us ds
  in

  let rec aux up down =
    if check_reflected true up down then List.length up
    else match down with [] -> 0 | d :: ds -> aux (d :: up) ds
  in

  aux [] map

let horizontal_reflections_smudge map =
  let rec nr_diffs_line xs ys =
    match (xs, ys) with
    | [], _ -> 0
    | _, [] -> 0
    | x :: xs, y :: ys ->
        if x = y then nr_diffs_line xs ys else 1 + nr_diffs_line xs ys
  in

  let rec nr_diffs up down =
    match (up, down) with
    | [], _ -> 0
    | _, [] -> 0
    | u :: us, d :: ds -> nr_diffs_line u d + nr_diffs us ds
  in

  let rec aux up down =
    if nr_diffs up down = 1 then List.length up
    else match down with [] -> 0 | d :: ds -> aux (d :: up) ds
  in

  aux [] map

let part_one maps =
  List.fold_left
    (fun acc map ->
      let horizontal = horizontal_reflections map in
      let vertical = horizontal_reflections (transpose map) in
      acc + vertical + (100 * horizontal))
    0 maps

let part_two maps =
  List.fold_left
    (fun acc map ->
      let horizontal = horizontal_reflections_smudge map in
      let vertical = horizontal_reflections_smudge (transpose map) in
      acc + vertical + (100 * horizontal))
    0 maps

let run () =
  let input = read_input "inputs/day_13.txt" in
  let solution_one = part_one input in
  let solution_two = part_two input in
  Printf.printf "Part One: %d\n" solution_one;
  Printf.printf "Part Two: %d\n" solution_two
