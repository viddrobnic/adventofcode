type tile = Empty | Galaxy

let print_grid grid =
  List.iter
    (fun row ->
      List.iter (fun c -> print_char (if c = Empty then '.' else '#')) row;
      print_newline ())
    grid

let read_input file_name =
  let file = open_in file_name in
  let lines = In_channel.input_lines file in
  let grid =
    List.map
      (fun line ->
        Seq.map
          (fun c -> if c = '.' then Empty else Galaxy)
          (String.to_seq line)
        |> List.of_seq)
      lines
  in

  (* Get coordinates of galaxies *)
  let galaxies =
    List.fold_left
      (fun (coords, y) row ->
        let coords, _ =
          List.fold_left
            (fun (coords, x) tile ->
              match tile with
              | Empty -> (coords, x + 1)
              | Galaxy -> ((x, y) :: coords, x + 1))
            (coords, 0) row
        in
        (coords, y + 1))
      ([], 0) grid
    |> fst
  in

  galaxies

let expand factor galaxies =
  (* Expand rows *)
  let galaxies, _, _ =
    List.sort (fun (_, y1) (_, y2) -> compare y1 y2) galaxies
    |> List.fold_left
         (fun (res, offset, last_y) (x, y) ->
           let space = y - last_y in
           let offset =
             if space <= 1 then offset else offset + ((space - 1) * (factor - 1))
           in
           ((x, y + offset) :: res, offset, y))
         ([], 0, 0)
  in

  (* Expand columns *)
  let galaxies, _, _ =
    List.sort (fun (x1, _) (x2, _) -> compare x1 x2) galaxies
    |> List.fold_left
         (fun (res, offset, last_x) (x, y) ->
           let space = x - last_x in
           let offset =
             if space <= 1 then offset else offset + ((space - 1) * (factor - 1))
           in
           ((x + offset, y) :: res, offset, x))
         ([], 0, 0)
  in
  galaxies

let solve factor galaxies =
  let rec sum_shortest_paths sum = function
    | [] -> sum
    | [ _ ] -> sum
    | (x1, y1) :: rest ->
        let new_sum =
          List.fold_left
            (fun acc (x2, y2) -> acc + (abs (x2 - x1) + abs (y2 - y1)))
            sum rest
        in
        sum_shortest_paths new_sum rest
  in
  sum_shortest_paths 0 (expand factor galaxies)

let part_one = solve 2
let part_two = solve 1_000_000

let run () =
  let input = read_input "inputs/day_11.txt" in
  let solution_one = part_one input in
  let solution_two = part_two input in
  Printf.printf "Part one: %d\n" solution_one;
  Printf.printf "Part two: %d\n" solution_two
