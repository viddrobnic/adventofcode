type point = { x : int; y : int }

let read_input () =
  let ic = open_in "inputs/day09.txt" in
  let lines = In_channel.input_lines ic in
  List.map
    (fun line ->
      let parts = String.split_on_char ',' line in
      match parts with
      | [ x; y ] -> { x = int_of_string x; y = int_of_string y }
      | _ -> failwith "invalid point")
    lines

let gen_pairs xs =
  let rec gen_pair x xs acc =
    match xs with [] -> acc | y :: ys -> gen_pair x ys ((x, y) :: acc)
  in

  let rec aux xs acc =
    match xs with [] -> acc | y :: ys -> aux ys (gen_pair y ys acc)
  in

  aux xs []

let area a b =
  let dx = a.x - b.x |> abs in
  let dy = a.y - b.y |> abs in
  (dx + 1) * (dy + 1)

let part_one =
  List.fold_left
    (fun acc (a, b) ->
      let a = area a b in
      max a acc)
    0

let part_two points pairs =
  let rec get_segments acc curr = function
    | [] -> acc
    | x :: xs -> get_segments ((x, curr) :: acc) x xs
  in
  let segments =
    get_segments [] (List.hd points) (List.tl points @ [ List.hd points ])
  in

  List.fold_left
    (fun acc (a, b) ->
      let ar = area a b in
      if acc >= ar then acc
      else
        let min_x = min a.x b.x in
        let max_x = max a.x b.x in
        let min_y = min a.y b.y in
        let max_y = max a.y b.y in

        let valid =
          List.for_all
            (fun (p1, p2) ->
              let s_x_min = min p1.x p2.x in
              let s_x_max = max p1.x p2.x in
              let s_y_min = min p1.y p2.y in
              let s_y_max = max p1.y p2.y in

              min_x >= s_x_max || max_x <= s_x_min || min_y >= s_y_max
              || max_y <= s_y_min)
            segments
        in

        if valid then ar else acc)
    0 pairs

let run () =
  let input = read_input () in
  let pairs = gen_pairs input in
  let p_one = part_one pairs in
  let p_two = part_two input pairs in
  Printf.printf "Part one: %d\n" p_one;
  Printf.printf "Part two: %d\n" p_two
