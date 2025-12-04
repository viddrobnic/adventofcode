let read_input () =
  let ic = open_in "inputs/day04.txt" in
  let lines = In_channel.input_lines ic in
  List.map
    (fun line ->
      String.to_seq line |> Seq.map (fun ch -> ch = '@') |> Iarray.of_seq)
    lines
  |> Iarray.of_list

let get input x y = Iarray.get (Iarray.get input y) x

let nieghbours =
  [ (-1, 0); (1, 0); (0, 1); (0, -1); (-1, 1); (-1, -1); (1, 1); (1, -1) ]

let get_nr_rolls input x y =
  List.fold_left
    (fun acc (dx, dy) ->
      let x2 = x + dx in
      let y2 = y + dy in
      if
        x2 >= 0
        && x2 < Iarray.length (Iarray.get input 0)
        && y2 >= 0
        && y2 < Iarray.length input
      then if get input x2 y2 then acc + 1 else acc
      else acc)
    0 nieghbours

let next input x y =
  if x = Iarray.length (Iarray.get input 0) - 1 then
    if y = Iarray.length input - 1 then None else Some (0, y + 1)
  else Some (x + 1, y)

let part_one input =
  let rec aux input x y acc =
    if not (get input x y) then
      match next input x y with None -> acc | Some (x, y) -> aux input x y acc
    else
      let nr_rolls = get_nr_rolls input x y in
      let new_acc = if nr_rolls < 4 then acc + 1 else acc in

      match next input x y with
      | None -> acc
      | Some (x, y) -> aux input x y new_acc
  in

  aux input 0 0 0

let print_map input =
  Iarray.iter
    (fun row ->
      Iarray.iter (fun c -> if c then print_char '@' else print_char '.') row;
      print_newline ())
    input

let part_two input =
  let rec remove input x y removed map row =
    let row, removed =
      if not (get input x y) then
        (Iarray.append row (Iarray.of_list [ false ]), removed)
      else
        let nr_rolls = get_nr_rolls input x y in
        if nr_rolls < 4 then
          (Iarray.append row (Iarray.of_list [ false ]), removed + 1)
        else (Iarray.append row (Iarray.of_list [ true ]), removed)
    in
    match next input x y with
    | None -> (Iarray.append map (Iarray.of_list [ row ]), removed)
    | Some (0, y) ->
        remove input 0 y removed
          (Iarray.append map (Iarray.of_list [ row ]))
          (Iarray.of_list [])
    | Some (x, y) -> remove input x y removed map row
  in

  let rec aux input acc =
    let map, removed =
      remove input 0 0 0 (Iarray.of_list []) (Iarray.of_list [])
    in
    if removed = 0 then acc else aux map (acc + removed)
  in

  aux input 0

let run () =
  let input = read_input () in
  let p_one = part_one input in
  let p_two = part_two input in
  Printf.printf "Part one: %d\n" p_one;
  Printf.printf "Part two: %d\n" p_two
