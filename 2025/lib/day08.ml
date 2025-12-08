type point = { x : int; y : int; z : int }

module OrderedPoint = struct
  type t = point

  let compare a b =
    match Stdlib.compare a.x b.x with
    | 0 -> (
        match Stdlib.compare a.y b.y with 0 -> Stdlib.compare a.z b.z | c -> c)
    | c -> c
end

module GraphMap = Map.Make (OrderedPoint)
module PointSet = Set.Make (OrderedPoint)

let dist a b =
  let dx = a.x - b.x in
  let dy = a.y - b.y in
  let dz = a.z - b.z in
  (dx * dx) + (dy * dy) + (dz * dz)

let read_input () =
  let ic = open_in "inputs/day08.txt" in
  let lines = In_channel.input_lines ic in
  List.map
    (fun line ->
      let parts = String.split_on_char ',' line in
      match parts with
      | [ x; y; z ] ->
          { x = int_of_string x; y = int_of_string y; z = int_of_string z }
      | _ -> failwith "invalid point")
    lines

let gen_pairs xs =
  let rec gen_pair x xs acc =
    match xs with
    | [] -> acc
    | y :: ys -> gen_pair x ys ((x, y, dist x y) :: acc)
  in

  let rec aux xs acc =
    match xs with [] -> acc | y :: ys -> aux ys (gen_pair y ys acc)
  in

  let pairs = aux xs [] in
  List.sort (fun (_, _, d1) (_, _, d2) -> d1 - d2) pairs

let connected_comps g f init =
  let rec fold f acc values =
    match values () with
    | Seq.Nil -> acc
    | Seq.Cons (x, xs) ->
        let acc, continue = f acc x in
        if continue then fold f acc xs else acc
  in

  let rec connected_comp g visited stack acc =
    match stack with
    | [] -> (acc, visited)
    | x :: xs ->
        if PointSet.find_opt x visited |> Option.is_some then
          connected_comp g visited xs acc
        else
          let neighs = GraphMap.find x g in
          connected_comp g (PointSet.add x visited) (xs @ neighs) (acc + 1)
  in

  let res, _ =
    fold
      (fun (acc, visited) node ->
        if PointSet.find_opt node visited |> Option.is_some then
          ((acc, visited), true)
        else
          let nr_nodes, visited = connected_comp g visited [ node ] 0 in
          let acc, continue = f acc nr_nodes in
          ((acc, visited), continue))
      (init, PointSet.empty)
      (GraphMap.to_seq g |> Seq.map fst)
  in
  res

let connect g p1 p2 =
  let g =
    GraphMap.update p1
      (fun existing ->
        match existing with
        | None -> Some [ p2 ]
        | Some neighs -> Some (p2 :: neighs))
      g
  in
  GraphMap.update p2
    (fun existing ->
      match existing with
      | None -> Some [ p1 ]
      | Some neighs -> Some (p1 :: neighs))
    g

let part_one pairs =
  let rec gen_graph g = function
    | [] -> g
    | (p1, p2, _) :: rest -> gen_graph (connect g p1 p2) rest
  in

  let graph = gen_graph GraphMap.empty (List.take 1000 pairs) in

  let comps = connected_comps graph (fun acc size -> (size :: acc, true)) [] in
  let sorted = List.sort (fun a b -> b - a) comps in
  let a = List.nth sorted 0 in
  let b = List.nth sorted 1 in
  let c = List.nth sorted 2 in
  a * b * c

let part_two points pairs =
  let rec aux g = function
    | [] -> failwith "graph is never a single component"
    | (p1, p2, _) :: rest ->
        let g = connect g p1 p2 in
        let nr_comps = connected_comps g (fun acc _ -> (acc + 1, acc < 2)) 0 in
        if nr_comps = 1 then p1.x * p2.x else aux g rest
  in

  let graph =
    List.fold_left (fun g p -> GraphMap.add p [] g) GraphMap.empty points
  in

  aux graph pairs

let run () =
  let input = read_input () in
  let pairs = gen_pairs input in
  let p_one = part_one pairs in
  let p_two = part_two input pairs in
  Printf.printf "Part one: %d\n" p_one;
  Printf.printf "Part two: %d\n" p_two
