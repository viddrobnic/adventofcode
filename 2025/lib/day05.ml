type interval = int * int
type input = { intervals : interval list; ids : int list }

let read_input () =
  let ic = open_in "inputs/day05.txt" in
  let lines = In_channel.input_lines ic in

  let rec split_lines input intervals ids is_interval =
    match input with
    | "" :: xs -> split_lines xs intervals ids false
    | x :: xs ->
        if is_interval then split_lines xs (x :: intervals) ids is_interval
        else split_lines xs intervals (x :: ids) is_interval
    | [] -> (intervals, ids)
  in
  let intervals, ids = split_lines lines [] [] true in

  let intervals =
    List.map
      (fun interval ->
        let left, right =
          match String.split_on_char '-' interval with
          | [ left; right ] -> (left, right)
          | _ -> failwith ("invalid interval: " ^ interval)
        in
        (int_of_string left, int_of_string right))
      intervals
  in

  let ids = List.map int_of_string ids in

  let intervals = List.sort (fun (a, _) (b, _) -> a - b) intervals in
  let rec group_intervals intervals acc (curr_left, curr_right) =
    match intervals with
    | [] -> List.rev ((curr_left, curr_right) :: acc)
    | (left, right) :: xs ->
        if left >= curr_left && left <= curr_right then
          group_intervals xs acc (curr_left, max right curr_right)
        else group_intervals xs ((curr_left, curr_right) :: acc) (left, right)
  in
  let intervals = group_intervals (List.tl intervals) [] (List.hd intervals) in

  { intervals; ids }

let part_one input =
  List.fold_left
    (fun acc id ->
      let contains =
        List.exists
          (fun (left, right) -> id >= left && id <= right)
          input.intervals
      in
      if contains then acc + 1 else acc)
    0 input.ids

let part_two input =
  List.fold_left
    (fun acc (left, right) -> acc + right - left + 1)
    0 input.intervals

let run () =
  let input = read_input () in
  let p_one = part_one input in
  let p_two = part_two input in
  Printf.printf "Part one: %d\n" p_one;
  Printf.printf "Part two: %d\n" p_two
