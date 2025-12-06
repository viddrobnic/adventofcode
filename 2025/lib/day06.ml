type operation = Mult | Add

let operation_start = function Mult -> 1 | Add -> 0
let operation_op = function Mult -> ( * ) | Add -> ( + )

let transpose m =
  let rec aux acc = function
    | [] | [] :: _ -> List.rev acc
    | rows ->
        let heads = List.map List.hd rows in
        let tails = List.map List.tl rows in
        aux (heads :: acc) tails
  in
  aux [] m

let read_input () =
  let ic = open_in "inputs/day06.txt" in
  In_channel.input_lines ic

let part_one input =
  let lines =
    List.map (fun line -> Str.split (Str.regexp "[ \t]+") line) input
    |> List.rev
  in

  let ops = List.hd lines in
  let numbers = List.tl lines in

  let ops =
    List.map
      (fun c ->
        match c with
        | "*" -> Mult
        | "+" -> Add
        | _ -> failwith "invalid operation")
      ops
  in

  let numbers = List.map (List.map int_of_string) numbers |> transpose in
  let input = List.combine ops numbers in

  let results =
    List.map
      (fun (op, numbers) ->
        List.fold_left (operation_op op) (operation_start op) numbers)
      input
  in
  List.fold_left ( + ) 0 results

let part_two input =
  let reversed = List.rev input in
  let ops = List.hd reversed in
  let lines = List.tl reversed |> List.rev in

  let rec find_operator idx =
    match ops.[idx] with
    | ' ' -> find_operator (idx - 1)
    | '*' -> (Mult, idx)
    | '+' -> (Add, idx)
    | _ -> failwith "invalid operator"
  in

  let num_from_col idx =
    List.fold_left
      (fun acc row ->
        let digit =
          if row.[idx] = ' ' then None
          else Some (int_of_char row.[idx] - int_of_char '0')
        in

        match digit with None -> acc | Some digit -> (acc * 10) + digit)
      0 lines
  in

  let rec fold op acc left right =
    if left > right then acc
    else
      let n = num_from_col left in
      fold op (op acc n) (left + 1) right
  in

  let rec solve acc right =
    if right <= 0 then acc
    else
      let op, left = find_operator right in
      let sol = fold (operation_op op) (operation_start op) left right in
      solve (acc + sol) (left - 2)
  in

  solve 0 (String.length ops - 1)

let run () =
  let input = read_input () in
  let p_one = part_one input in
  let p_two = part_two input in
  Printf.printf "Part one: %d\n" p_one;
  Printf.printf "Part two: %d\n" p_two
