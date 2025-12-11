module Graph = Map.Make (String)

let read_input () =
  let ic = open_in "inputs/day11.txt" in
  let lines = In_channel.input_lines ic in

  List.fold_left
    (fun acc line ->
      let parts = String.split_on_char ':' line in
      let node = List.hd parts in

      let neighs = List.hd (List.tl parts) in
      let neighs = String.sub neighs 1 (String.length neighs - 1) in
      let neighs = String.split_on_char ' ' neighs in

      Graph.add node neighs acc)
    Graph.empty lines

let part_one g =
  let memo = Hashtbl.create 128 in
  let rec aux u t =
    if u = t then 1
    else
      match Hashtbl.find_opt memo u with
      | Some res -> res
      | None ->
          let neighs =
            match Graph.find_opt u g with None -> [] | Some xs -> xs
          in
          let res = List.fold_left (fun acc v -> acc + aux v t) 0 neighs in
          Hashtbl.add memo u res;
          res
  in

  aux "you" "out"

let part_two g =
  let memo = Hashtbl.create 128 in
  let rec aux u t fft dac =
    if u = t && fft && dac then 1
    else
      let fft = fft || u = "fft" in
      let dac = dac || u = "dac" in

      match Hashtbl.find_opt memo (u, fft, dac) with
      | Some res -> res
      | None ->
          let neighs =
            match Graph.find_opt u g with None -> [] | Some xs -> xs
          in

          let res =
            List.fold_left (fun acc v -> acc + aux v t fft dac) 0 neighs
          in
          Hashtbl.add memo (u, fft, dac) res;
          res
  in

  aux "svr" "out" false false

let run () =
  let input = read_input () in
  let p_one = part_one input in
  let p_two = part_two input in
  Printf.printf "Part one: %d\n" p_one;
  Printf.printf "Part two: %d\n" p_two
