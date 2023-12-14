type tile = Empty | Round | Cube

let read_input filename =
  let file = open_in filename in
  let lines = In_channel.input_lines file in

  let height = List.length lines in
  let width = String.length (List.hd lines) in
  let map = Array.make_matrix height width Empty in

  List.iteri
    (fun y line ->
      String.iteri
        (fun x c ->
          map.(y).(x) <-
            (match c with
            | '.' -> Empty
            | 'O' -> Round
            | '#' -> Cube
            | _ -> failwith "Invalid input"))
        line)
    lines;

  map

let copy_map map =
  let height = Array.length map in
  let width = Array.length map.(0) in
  let new_map = Array.make_matrix height width Empty in
  Array.iteri
    (fun y row -> Array.iteri (fun x t -> new_map.(y).(x) <- t) row)
    map;
  new_map

let print_map map =
  Array.iter
    (fun row ->
      Array.iter
        (fun t ->
          print_string
            (match t with Empty -> "." | Round -> "O" | Cube -> "#"))
        row;
      print_newline ())
    map

let rec move dx dy last_x last_y x y map =
  let height = Array.length map in
  let width = Array.length map.(0) in
  if x < 0 || x >= width || y < 0 || y >= height then ()
  else
    match map.(y).(x) with
    | Empty -> move dx dy last_x last_y (x + dx) (y + dy) map
    | Cube -> move dx dy x y (x + dx) (y + dy) map
    | Round ->
        map.(y).(x) <- Empty;
        map.(last_y + dy).(last_x + dx) <- Round;
        move dx dy (last_x + dx) (last_y + dy) (x + dx) (y + dy) map

let move_north map =
  let width = Array.length map.(0) in
  List.init width (fun x -> x) |> List.iter (fun x -> move 0 1 x (-1) x 0 map)

let move_west map =
  let height = Array.length map in
  List.init height (fun y -> y) |> List.iter (fun y -> move 1 0 (-1) y 0 y map)

let move_south map =
  let width = Array.length map.(0) in
  let height = Array.length map in
  List.init width (fun x -> x)
  |> List.iter (fun x -> move 0 (-1) x height x (height - 1) map)

let move_east map =
  let width = Array.length map.(0) in
  let height = Array.length map in
  List.init height (fun y -> y)
  |> List.iter (fun y -> move (-1) 0 width y (width - 1) y map)

let calculate_load map =
  Array.fold_left
    (fun (row_score, score) row ->
      let count =
        Array.fold_left
          (fun acc t -> match t with Round -> acc + 1 | _ -> acc)
          0 row
      in
      (row_score - 1, score + (row_score * count)))
    (Array.length map, 0)
    map
  |> snd

let part_one input =
  let map = copy_map input in
  move_north map;
  calculate_load map

let part_two input =
  let cycle map =
    move_north map;
    move_west map;
    move_south map;
    move_east map
  in
  let map = copy_map input in
  let num = 1_000_000_000 in
  let memo = Hashtbl.create 512 in
  let memo_map = Hashtbl.create 512 in

  let rec aux n =
    cycle map;
    if n = num then calculate_load map
    else
      match Hashtbl.find_opt memo map with
      | Some m ->
          let cycle_length = n - m in
          let rest = (num - n) mod cycle_length in
          let m = Hashtbl.find memo_map (m + rest) in
          calculate_load m
      | None ->
          Hashtbl.add memo (copy_map map) n;
          Hashtbl.add memo_map n (copy_map map);
          aux (n + 1)
  in

  aux 1

let run () =
  let input = read_input "inputs/day_14.txt" in
  let solution_one = part_one input in
  let solution_two = part_two input in
  Printf.printf "Part One: %d\n" solution_one;
  Printf.printf "Part Two: %d\n" solution_two
