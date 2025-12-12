(* This day contains a shortcut. I compute shape sizes by hand,
   and input is truncated to contain only the list of regions *)

let shapes = [ 7; 7; 7; 5; 6; 7 ]

let read_input () =
  let ic = open_in "inputs/day12.txt" in
  let lines = In_channel.input_lines ic in

  List.map
    (fun line ->
      let parts = String.split_on_char ':' line in

      let size_part = List.hd parts in
      let size_parts = String.split_on_char 'x' size_part in
      let w = List.hd size_parts |> int_of_string in
      let h = List.nth size_parts 1 |> int_of_string in
      let size = w * h in

      let counts = List.nth parts 1 in
      let counts = String.sub counts 1 (String.length counts - 1) in
      let counts = String.split_on_char ' ' counts |> List.map int_of_string in

      (size, counts))
    lines

let part_one =
  List.fold_left
    (fun acc (size, counts) ->
      let act_size =
        List.fold_left2
          (fun acc shape count -> acc + (shape * count))
          0 shapes counts
      in
      if act_size <= size then acc + 1 else acc)
    0

let run () =
  let input = read_input () in
  let p_one = part_one input in
  Printf.printf "Part one: %d\n" p_one
