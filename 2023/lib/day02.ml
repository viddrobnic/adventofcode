type game_iteration = { red : int; green : int; blue : int }
type game = { id : int; iterations : game_iteration list }

let show_game_iteration iteration =
  Printf.sprintf "Red: %d, Green: %d, Blue: %d" iteration.red iteration.green
    iteration.blue

let parse_line line =
  match String.split_on_char ':' line with
  | [ id; iterations ] ->
      let get_game_id game_id =
        match String.split_on_char ' ' game_id with
        | [ _; id ] -> int_of_string id
        | _ -> failwith ("Invalid input: " ^ line)
      in
      let game_id = get_game_id id in

      let get_iterations input =
        let parse_iteration iteration =
          String.split_on_char ',' iteration
          |> List.fold_left
               (fun acc x ->
                 match String.trim x |> String.split_on_char ' ' with
                 | [ value; "red" ] -> { acc with red = int_of_string value }
                 | [ value; "green" ] ->
                     { acc with green = int_of_string value }
                 | [ value; "blue" ] -> { acc with blue = int_of_string value }
                 | _ -> failwith ("invalid iteration input: " ^ x))
               { red = 0; green = 0; blue = 0 }
        in
        String.split_on_char ';' input |> List.map parse_iteration
      in
      let iterations = get_iterations iterations in

      { id = game_id; iterations }
  | _ -> failwith ("Invalid input: " ^ line)

let read_input () =
  let ic = open_in "inputs/day_02.txt" in
  In_channel.input_lines ic |> List.map parse_line

let part_one input =
  List.filter
    (fun game ->
      List.for_all
        (fun iteration ->
          iteration.red <= 12 && iteration.green <= 13 && iteration.blue <= 14)
        game.iterations)
    input
  |> List.fold_left (fun acc game -> acc + game.id) 0

let part_two input =
  let powers =
    List.map
      (fun game ->
        let min_cubes =
          List.fold_left
            (fun acc iteration ->
              {
                red = max acc.red iteration.red;
                green = max acc.green iteration.green;
                blue = max acc.blue iteration.blue;
              })
            { red = 0; green = 0; blue = 0 }
            game.iterations
        in
        min_cubes.red * min_cubes.green * min_cubes.blue)
      input
  in
  List.fold_left ( + ) 0 powers

let run () =
  let input = read_input () in
  let result_one = part_one input in
  let result_two = part_two input in
  print_endline ("Part One: " ^ string_of_int result_one);
  print_endline ("Part Two: " ^ string_of_int result_two)
