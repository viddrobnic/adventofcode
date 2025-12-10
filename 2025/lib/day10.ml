type machine = { lights : int; schematics : int list; joltages : int list }

module IntSet = Set.Make (Int)

let read_input () =
  let ic = open_in "inputs/day10.txt" in
  let lines = In_channel.input_lines ic in

  let parse_lights lights =
    let lights = String.sub lights 1 (String.length lights - 2) in
    List.fold_left
      (fun acc ch -> if ch = '.' then acc lsl 1 else (acc lsl 1) lor 1)
      0
      (String.to_seq lights |> List.of_seq |> List.rev)
  in

  let parse_joltages joltages =
    let joltages = String.sub joltages 1 (String.length joltages - 2) in
    List.map int_of_string (String.split_on_char ',' joltages)
  in

  let parse_schematics =
    List.map (fun s ->
        let schems = String.sub s 1 (String.length s - 2) in
        let parts = String.split_on_char ',' schems in
        List.fold_left
          (fun acc n ->
            let n = int_of_string n in
            let mask = 1 lsl n in
            acc lor mask)
          0 parts)
  in

  List.map
    (fun line ->
      let parts = String.split_on_char ' ' line in
      let lights = parse_lights (List.hd parts) in

      let parts = List.tl parts |> List.rev in
      let joltages = parse_joltages (List.hd parts) in

      let schematics = parse_schematics (List.tl parts) in

      { lights; schematics; joltages })
    lines

let part_one input =
  let rec nr_presses m visited q =
    let state, n = Queue.take q in
    if state = m.lights then n
    else if IntSet.find_opt state visited |> Option.is_some then
      nr_presses m visited q
    else
      let visited = IntSet.add state visited in
      List.iter (fun mask -> Queue.add (state lxor mask, n + 1) q) m.schematics;
      nr_presses m visited q
  in

  List.fold_left
    (fun acc m ->
      let q = Queue.create () in
      Queue.add (0, 0) q;
      let n = nr_presses m IntSet.empty q in
      acc + n)
    0 input

let run () =
  let input = read_input () in
  let p_one = part_one input in
  Printf.printf "Part one: %d\n" p_one
