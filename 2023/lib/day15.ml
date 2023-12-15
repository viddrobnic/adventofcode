type operation = Remove | Insert of int
type step = { label : string; hash : int; operation : operation }
type lense = { focal_length : int; label : string }

let read_input filename =
  let file = open_in filename in
  match In_channel.input_line file with
  | Some line -> String.split_on_char ',' line
  | None -> []

let hash input =
  String.fold_left (fun acc c -> (acc + Char.code c) * 17 mod 256) 0 input

let part_one input = List.fold_left (fun acc x -> acc + hash x) 0 input

let part_two input =
  let steps =
    List.map
      (fun step ->
        match String.split_on_char '=' step with
        | [ label; focal_length ] ->
            {
              label;
              hash = hash label;
              operation = Insert (int_of_string focal_length);
            }
        | _ ->
            let label = String.sub step 0 (String.length step - 1) in
            { label; hash = hash label; operation = Remove })
      input
  in

  let add_or_replace_lense lense lenses =
    if List.exists (fun l -> l.label = lense.label) lenses then
      List.map (fun l -> if l.label = lense.label then lense else l) lenses
    else lenses @ [ lense ]
  in

  let boxes = Array.make 256 [] in

  List.iter
    (fun step ->
      match step.operation with
      | Remove ->
          boxes.(step.hash) <-
            List.filter (fun x -> x.label <> step.label) boxes.(step.hash)
      | Insert focal_length ->
          boxes.(step.hash) <-
            add_or_replace_lense
              { label = step.label; focal_length }
              boxes.(step.hash))
    steps;

  Array.fold_left
    (fun (box, score) lenses ->
      let box_score =
        List.fold_left
          (fun (slot, score) lense ->
            (slot + 1, score + (box * slot * lense.focal_length)))
          (1, 0) lenses
        |> snd
      in
      (box + 1, score + box_score))
    (1, 0) boxes
  |> snd

let run () =
  let input = read_input "inputs/day_15.txt" in
  let result_one = part_one input in
  let result_two = part_two input in
  Printf.printf "Part One: %d\n" result_one;
  Printf.printf "Part Two: %d\n" result_two
