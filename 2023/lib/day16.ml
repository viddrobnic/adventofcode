type tile =
  | Empty
  | VerticalSplitter
  | HorizontalSlitter
  | LeftMirror
  | RightMirror

type direction = Left | Right | Up | Down

module TileSet = Set.Make (struct
  type t = int * int

  let compare = compare
end)

module DirectionTileSet = Set.Make (struct
  type t = direction * int * int

  let compare = compare
end)

let read_input filename =
  let file = open_in filename in
  let lines = In_channel.input_lines file in

  let height = List.length lines in
  let width = String.length (List.hd lines) in

  let grid = Array.make_matrix height width Empty in

  List.iteri
    (fun y line ->
      String.iteri
        (fun x c ->
          let tile =
            match c with
            | '.' -> Empty
            | '|' -> VerticalSplitter
            | '-' -> HorizontalSlitter
            | '\\' -> LeftMirror
            | '/' -> RightMirror
            | _ -> failwith "Invalid character"
          in
          grid.(y).(x) <- tile)
        line)
    lines;

  grid

let neighbours tile direction x y =
  match (direction, tile) with
  | Right, VerticalSplitter -> [ (Up, x, y - 1); (Down, x, y + 1) ]
  | Right, LeftMirror -> [ (Down, x, y + 1) ]
  | Right, RightMirror -> [ (Up, x, y - 1) ]
  | Right, _ -> [ (Right, x + 1, y) ]
  | Left, VerticalSplitter -> [ (Up, x, y - 1); (Down, x, y + 1) ]
  | Left, LeftMirror -> [ (Up, x, y - 1) ]
  | Left, RightMirror -> [ (Down, x, y + 1) ]
  | Left, _ -> [ (Left, x - 1, y) ]
  | Up, HorizontalSlitter -> [ (Left, x - 1, y); (Right, x + 1, y) ]
  | Up, LeftMirror -> [ (Left, x - 1, y) ]
  | Up, RightMirror -> [ (Right, x + 1, y) ]
  | Up, _ -> [ (Up, x, y - 1) ]
  | Down, HorizontalSlitter -> [ (Left, x - 1, y); (Right, x + 1, y) ]
  | Down, LeftMirror -> [ (Right, x + 1, y) ]
  | Down, RightMirror -> [ (Left, x - 1, y) ]
  | Down, _ -> [ (Down, x, y + 1) ]

let count_energised_tiles grid start_dir start_x start_y =
  let height = Array.length grid in
  let width = Array.length grid.(0) in

  let queue = Queue.create () in

  Queue.push (start_dir, start_x, start_y) queue;

  let rec bfs seen visited =
    if Queue.is_empty queue then visited
    else
      let direction, x, y = Queue.pop queue in
      if DirectionTileSet.mem (direction, x, y) seen then bfs seen visited
      else
        let seen = DirectionTileSet.add (direction, x, y) seen in
        let visited = TileSet.add (x, y) visited in

        let to_visit =
          neighbours grid.(y).(x) direction x y
          |> List.filter (fun (_, x, y) ->
                 x >= 0 && x < width && y >= 0 && y < height)
        in

        List.iter (fun elt -> Queue.push elt queue) to_visit;

        bfs seen visited
  in

  let visited = bfs DirectionTileSet.empty TileSet.empty in
  TileSet.cardinal visited

let part_one grid = count_energised_tiles grid Right 0 0

let part_two grid =
  let height = Array.length grid in
  let width = Array.length grid.(0) in

  let up = List.init width (fun x -> (Down, x, 0)) in
  let right = List.init height (fun y -> (Left, width - 1, y)) in
  let down = List.init width (fun x -> (Up, x, height - 1)) in
  let left = List.init height (fun y -> (Right, 0, y)) in

  let starting_points = up @ right @ down @ left in
  List.map
    (fun (direction, x, y) -> count_energised_tiles grid direction x y)
    starting_points
  |> List.fold_left max 0

let run () =
  let input = read_input "inputs/day_16.txt" in
  let solution_one = part_one input in
  let solution_two = part_two input in
  Printf.printf "Part One: %d\n" solution_one;
  Printf.printf "Part Two: %d\n" solution_two
