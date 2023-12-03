type number = { value : int; start_index : int; end_index : int }
type field = Number of number | Symbol of char | Empty

let print_map =
  Array.iter (fun row ->
      Array.iteri
        (fun i field ->
          match field with
          | Number n -> if n.start_index == i then print_int n.value else ()
          | Symbol s -> print_char s
          | Empty -> print_char '.')
        row;
      print_newline ())

let read_input file_name =
  let ic = open_in file_name in
  let lines = In_channel.input_lines ic in
  let height = List.length lines in
  let width = String.length (List.hd lines) in
  let map = Array.make_matrix height width Empty in

  (* Parse the input into a matrix of fields *)
  List.iteri
    (fun y line ->
      (* auxiliary function to add a Number type into the map matrix  *)
      let add_number n x =
        List.init (x - n.start_index) (fun i -> i + n.start_index)
        |> List.iter (fun i ->
               Array.set (Array.get map y) i (Number { n with end_index = x }))
      in

      let final =
        String.fold_left
          (fun acc c ->
            match (acc, c) with
            | (x, None), '.' ->
                Array.set (Array.get map y) x Empty;
                (x + 1, None)
            | (x, Some n), '.' ->
                add_number n x;
                Array.set (Array.get map y) x Empty;
                (x + 1, None)
            | (x, None), '0' .. '9' ->
                ( x + 1,
                  Some
                    {
                      value = Char.code c - Char.code '0';
                      start_index = x;
                      end_index = -1;
                    } )
            | (x, Some n), '0' .. '9' ->
                ( x + 1,
                  Some
                    {
                      n with
                      value = (n.value * 10) + Char.code c - Char.code '0';
                    } )
            | (x, None), c ->
                Array.set (Array.get map y) x (Symbol c);
                (x + 1, None)
            | (x, Some n), c ->
                add_number n x;
                Array.set (Array.get map y) x (Symbol c);
                (x + 1, None))
          (0, None) line
      in
      (* Handle number at the end of the line *)
      match final with x, Some n -> add_number n x | _ -> ())
    lines;

  map

let diff =
  [ (0, 1); (1, 0); (1, 1); (-1, 0); (0, -1); (-1, -1); (1, -1); (-1, 1) ]

let part_one input =
  let height = Array.length input in
  let width = Array.length (Array.get input 0) in

  let rec sum_row x y acc =
    if y >= height then acc
    else
      let row = Array.get input y in

      if x >= width then acc
      else
        match Array.get row x with
        | Number n ->
            (* x coordinates that the number spans *)
            let xs =
              List.init (n.end_index - n.start_index) (fun i ->
                  i + n.start_index)
            in

            (* (x, y) pairs of coordinates that the number spans,
               filtered to be only the ones inside the bounds *)
            let coords =
              List.map
                (fun x -> List.map (fun (dx, dy) -> (x + dx, y + dy)) diff)
                xs
              |> List.flatten
              |> List.filter (fun (x, y) ->
                     x >= 0 && y >= 0 && x < width && y < height)
            in

            (* Check if any of the coordinates is adjecant to a symbol *)
            let has_symbol =
              List.exists
                (fun (x, y) ->
                  match Array.get (Array.get input y) x with
                  | Symbol _ -> true
                  | _ -> false)
                coords
            in

            if has_symbol then sum_row n.end_index y (acc + n.value)
            else sum_row n.end_index y acc
        | _ -> sum_row (x + 1) y acc
  in

  (* Iterate over all rows and sum the numbers *)
  List.init height (fun i -> i)
  |> List.fold_left (fun acc y -> sum_row 0 y acc) 0

let part_two input =
  let height = Array.length input in
  let width = Array.length (Array.get input 0) in

  let rec sum_row x y acc =
    if y >= height then acc
    else
      let row = Array.get input y in

      if x >= width then acc
      else
        match Array.get row x with
        | Symbol '*' -> (
            (* Auxiliary function that gets number adjecant to the gear *)
            let rec get_numbers diffs visited acc =
              match diffs with
              | [] -> acc
              | (dx, dy) :: tl ->
                  let x = x + dx in
                  let y = y + dy in
                  if x >= 0 && y >= 0 && x < width && y < height then
                    (* Check if this coordinate was already visited *)
                    if List.exists (fun (x', y') -> x == x' && y == y') visited
                    then
                      (* Skip this coordinate if visited *)
                      get_numbers tl visited acc
                    else
                      (* Check if this coordinate is a number *)
                      match Array.get (Array.get input y) x with
                      | Number n ->
                          (* x coordinates that the number spans *)
                          let xs =
                            List.init (n.end_index - n.start_index) (fun i ->
                                i + n.start_index)
                          in
                          (* (x, y) pairs of coordinates that the number spans *)
                          let coords = List.map (fun x -> (x, y)) xs in

                          (* Recursively get the numbers adjecant to the gear,
                             not checking the current number again *)
                          get_numbers tl (coords @ visited) (n.value :: acc)
                      | _ -> get_numbers tl visited acc
                  else get_numbers tl visited acc
            in
            (* Get the numbers adjecant to the gear *)
            let gears = get_numbers diff [] [] in

            (* Check if there are exactly two gears and multiply them *)
            match gears with
            | [ gear1; gear2 ] -> sum_row (x + 1) y (acc + (gear1 * gear2))
            | _ -> sum_row (x + 1) y acc)
        | _ -> sum_row (x + 1) y acc
  in

  (* Iterate over all rows and sum the gear ratios *)
  List.init height (fun i -> i)
  |> List.fold_left (fun acc y -> sum_row 0 y acc) 0

let run () =
  let input = read_input "inputs/day_03.txt" in
  let result_one = part_one input in
  let result_two = part_two input in
  print_endline ("Part one: " ^ string_of_int result_one);
  print_endline ("Part two: " ^ string_of_int result_two)
