type direction = Left | Right

let string_of_direction = function Left -> "L" | Right -> "R"

let read_input file_name =
  let ic = open_in file_name in
  let lines = In_channel.input_lines ic in
  let instructions, map_lines =
    match lines with
    | steps :: _ :: map ->
        let q = Queue.create () in
        String.iter
          (fun c ->
            let d =
              match c with
              | 'L' -> Left
              | 'R' -> Right
              | _ -> failwith ("Invalid input: " ^ Char.escaped c)
            in
            Queue.push d q)
          steps;
        (q, map)
    | _ -> failwith "Invalid input"
  in
  let map = Hashtbl.create 100 in
  List.iter
    (fun line ->
      let parts = String.split_on_char '=' line in
      match parts with
      | [ key; value ] -> (
          let cleaned = Str.global_replace (Str.regexp "[\\(\\)]") "" value in
          let parts = String.split_on_char ',' cleaned in
          match parts with
          | [ left; right ] ->
              Hashtbl.add map (String.trim key)
                (String.trim left, String.trim right)
          | _ -> failwith ("Invalid line: " ^ line))
      | _ -> failwith ("Invalid line: " ^ line))
    map_lines;

  (instructions, map)

let part_one input =
  let rec nr_steps steps position directions map =
    if position = "ZZZ" then steps
    else
      let instructions = Hashtbl.find map position in
      let direction = Queue.pop directions in
      Queue.push direction directions;
      let next_position =
        match direction with
        | Left -> fst instructions
        | Right -> snd instructions
      in
      nr_steps (steps + 1) next_position directions map
  in

  nr_steps 0 "AAA" (fst input) (snd input)

let part_two input =
  let ends_with str c = String.get str (String.length str - 1) = c in
  let rec nr_steps steps position directions map =
    if ends_with position 'Z' then steps
    else
      let direction = Queue.pop directions in
      Queue.push direction directions;
      let instruction = Hashtbl.find map position in
      let new_position =
        match direction with
        | Left -> fst instruction
        | Right -> snd instruction
      in

      nr_steps (steps + 1) new_position directions map
  in

  let start_positions =
    Hashtbl.fold
      (fun k _ acc -> if ends_with k 'A' then k :: acc else acc)
      (snd input) []
  in
  let lengths =
    List.map (fun p -> nr_steps 0 p (fst input) (snd input)) start_positions
  in

  let rec gcd a b =
    if b > a then gcd b a else if b = 0 then a else gcd b (a mod b)
  in

  List.fold_left (fun acc steps -> acc * steps / gcd acc steps) 1 lengths

let run () =
  let input = read_input "inputs/day_08.txt" in
  let solution_one = part_one input in
  let solution_two = part_two input in
  print_endline ("Part One: " ^ string_of_int solution_one);
  print_endline ("Part Two: " ^ string_of_int solution_two)
