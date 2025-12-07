let read_input () =
  let ic = open_in "inputs/day07.txt" in
  In_channel.input_lines ic

let part_one input =
  let start_idx = String.index (List.hd input) 'S' in

  let add xs x =
    match xs with [] -> [ x ] | y :: _ when x = y -> xs | _ -> x :: xs
  in

  let rec new_beams line acc splits = function
    | [] -> (splits, List.rev acc)
    | b :: bs ->
        if line.[b] = '^' then
          let left = b - 1 in
          let right = b + 1 in
          let acc = add acc left in
          let acc = add acc right in
          new_beams line acc (splits + 1) bs
        else new_beams line (add acc b) splits bs
  in

  let rec solve lines beams res =
    match lines with
    | [] -> res
    | line :: rest ->
        let splits, n_beams = new_beams line [] 0 beams in
        solve rest n_beams (res + splits)
  in
  solve (List.tl input) [ start_idx ] 0

let part_two input =
  let start_idx = String.index (List.hd input) 'S' in

  let memo = Hashtbl.create 128 in
  let rec solve lines idx =
    match Hashtbl.find_opt memo (lines, idx) with
    | Some res -> res
    | None -> (
        match lines with
        | [] -> 1
        | line :: rest ->
            let res =
              if line.[idx] = '^' then
                solve rest (idx - 1) + solve rest (idx + 1)
              else solve rest idx
            in
            Hashtbl.add memo (lines, idx) res;
            res)
  in

  solve (List.tl input) start_idx

let run () =
  let input = read_input () in
  let p_one = part_one input in
  let p_two = part_two input in
  Printf.printf "Part one: %d\n" p_one;
  Printf.printf "Part two: %d\n" p_two
