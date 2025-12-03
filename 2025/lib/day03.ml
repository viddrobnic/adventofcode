let read_input () =
  let ic = open_in "inputs/day03.txt" in
  let lines = In_channel.input_lines ic in
  List.map
    (fun row ->
      row |> String.to_seq |> Seq.map (fun c -> Char.code c - Char.code '0'))
    lines

let find_max xs =
  let m, m_idx, _ =
    Seq.fold_left
      (fun (m, m_idx, idx) x ->
        if x > m then (x, idx, idx + 1) else (m, m_idx, idx + 1))
      (0, 0, 0) xs
  in
  (m, m_idx)

let rec max_number digits n acc =
  if n = 0 then acc
  else
    let m, m_idx = find_max (Seq.take (Seq.length digits - n + 1) digits) in
    max_number (Seq.drop (m_idx + 1) digits) (n - 1) ((acc * 10) + m)

let solve n =
  List.fold_left
    (fun acc row ->
      let n = max_number row n 0 in
      acc + n)
    0

let run () =
  let input = read_input () in
  let p_one = solve 2 input in
  let p_two = solve 12 input in
  Printf.printf "Part one: %d\n" p_one;
  Printf.printf "Part two: %d\n" p_two
