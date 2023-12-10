module CoordSet = Set.Make (struct
  type t = int * int

  let compare = compare
end)

let read_input filename =
  let file = open_in filename in
  let lines = In_channel.input_lines file in
  let height = List.length lines in
  let width = String.length (List.hd lines) in
  let grid = Array.make_matrix height width '.' in

  List.iteri
    (fun y line -> String.iteri (fun x c -> grid.(y).(x) <- c) line)
    lines;

  grid

let find_start grid =
  Array.find_mapi
    (fun y row ->
      Array.find_mapi (fun x c -> if c = 'S' then Some (x, y) else None) row)
    grid

let start_neigbours grid x y =
  let height = Array.length grid in
  let width = Array.length grid.(0) in
  (* x, y, valid_characters *)
  [
    (x - 1, y, [ '-'; 'L'; 'F' ]);
    (x + 1, y, [ '-'; 'J'; '7' ]);
    (x, y - 1, [ '|'; '7'; 'F' ]);
    (x, y + 1, [ '|'; 'J'; 'L' ]);
  ]
  |> List.filter_map (fun (x, y, valid) ->
         if x < 0 || y < 0 || x >= width || y >= height then None
         else if List.mem grid.(y).(x) valid then Some (x, y)
         else None)

let neighbours grid x y =
  match grid.(y).(x) with
  | '|' -> [ (x, y - 1); (x, y + 1) ]
  | '-' -> [ (x - 1, y); (x + 1, y) ]
  | 'L' -> [ (x, y - 1); (x + 1, y) ]
  | 'J' -> [ (x, y - 1); (x - 1, y) ]
  | '7' -> [ (x, y + 1); (x - 1, y) ]
  | 'F' -> [ (x, y + 1); (x + 1, y) ]
  | _ -> failwith ("Invalid character: " ^ Char.escaped grid.(y).(x))

let part_one grid =
  let start_x, start_y =
    match find_start grid with
    | Some (x, y) -> (x, y)
    | None -> failwith "No start found"
  in
  let s_neighbours = start_neigbours grid start_x start_y in

  let rec dfs x y polygon =
    if x = start_x && y = start_y then polygon
    else
      let nghs =
        neighbours grid x y
        |> List.filter (fun (x, y) ->
               let prev_x, prev_y = List.hd polygon in
               x != prev_x || y != prev_y)
      in
      match nghs with
      | [ (next_x, next_y) ] -> dfs next_x next_y ((x, y) :: polygon)
      | _ -> failwith "Invalid polygon"
  in

  let x, y = List.hd s_neighbours in
  let polygon = dfs x y [ (start_x, start_y) ] in
  (polygon, List.length polygon / 2)

let part_two polygon =
  let sum, _, _ =
    List.fold_left
      (fun (sum, x0, y0) (x1, y1) -> (sum + ((y0 + y1) * (x0 - x1)), x1, y1))
      (0, fst (List.hd polygon), snd (List.hd polygon))
      (List.tl polygon @ [ List.hd polygon ])
  in
  let area = abs sum / 2 in
  (* Pick's theorem *)
  area + 1 - (List.length polygon / 2)

let run () =
  let input = read_input "inputs/day_10.txt" in
  let polygon, solution_one = part_one input in
  let solution_two = part_two polygon in
  print_endline ("Part One: " ^ string_of_int solution_one);
  print_endline ("Part Two: " ^ string_of_int solution_two)
