type range = { source_start : int; destination_start : int; length : int }

let read_input file_name =
  let ic = open_in file_name in
  let lines = In_channel.input_lines ic in

  let parse_seeds line =
    line |> String.split_on_char ' ' |> List.tl |> List.map int_of_string
  in
  (* First line is the seeds *)
  let seeds = parse_seeds (List.hd lines) in
  (* Skip the first two lines, since second line is empty *)
  let lines = List.tl (List.tl lines) in

  let rec parse_maps res acc lines =
    match lines with
    | [] -> List.rev (List.rev acc :: res)
    | line :: rest -> (
        match line with
        | "" -> parse_maps (List.rev acc :: res) [] (List.tl rest)
        | _ -> (
            let parts = String.split_on_char ' ' line in
            match parts with
            | [ destination_start; source_start; length ] ->
                let range =
                  {
                    source_start = int_of_string source_start;
                    destination_start = int_of_string destination_start;
                    length = int_of_string length;
                  }
                in
                parse_maps res (range :: acc) rest
            | _ -> failwith ("Invalid line: " ^ line)))
  in
  let maps = parse_maps [] [] (List.tl lines) in

  (seeds, maps)

let part_one (seeds, maps) =
  let apply_map source map =
    let mapped_source =
      List.find_map
        (fun range ->
          if
            source >= range.source_start
            && source < range.source_start + range.length
          then Some (source - range.source_start + range.destination_start)
          else None)
        map
    in
    match mapped_source with Some x -> x | None -> source
  in

  let locations =
    List.map (fun seed -> List.fold_left apply_map seed maps) seeds
  in
  List.fold_left min max_int locations

let part_two (seeds, maps) =
  let rec seeds_to_range seeds acc =
    match seeds with
    | [] -> List.rev acc
    | x :: y :: rest -> seeds_to_range rest ((x, y) :: acc)
    | _ -> failwith "Invalid seeds"
  in
  let seeds = seeds_to_range seeds [] in

  let range_intersection start1 length1 start2 length2 =
    let end1 = start1 + length1 - 1 in
    let end2 = start2 + length2 - 1 in
    let start = max start1 start2 in
    let endi = min end1 end2 in

    if start <= endi then Some (start, endi - start + 1) else None
  in

  let apply_map (source_start, source_length) map =
    let mapped_sources =
      List.filter_map
        (fun range ->
          let intersection =
            range_intersection source_start source_length range.source_start
              range.length
          in
          match intersection with
          | None -> None
          | Some (start, length) ->
              Some
                {
                  source_start = start;
                  destination_start =
                    start - range.source_start + range.destination_start;
                  length;
                })
        map
    in

    (* Handle not being in any of the map intervals *)
    let mapped_interval =
      List.fold_left
        (fun acc range ->
          match acc with
          | Some (minm, maxm) ->
              Some
                ( min minm range.source_start,
                  max maxm (range.source_start + range.length - 1) )
          | None ->
              Some (range.source_start, range.source_start + range.length - 1))
        None mapped_sources
    in
    let mapped_sources =
      match mapped_interval with
      | None ->
          [
            {
              source_start;
              destination_start = source_start;
              length = source_length;
            };
          ]
      | Some (min_mapped, max_mapped) ->
          let mapped_sources =
            if source_start < min_mapped then
              {
                source_start;
                destination_start = source_start;
                length = min_mapped - source_start;
              }
              :: mapped_sources
            else mapped_sources
          in

          if source_start + source_length - 1 > max_mapped then
            {
              source_start = max_mapped + 1;
              destination_start = max_mapped + 1;
              length = source_start + source_length - 1 - max_mapped;
            }
            :: mapped_sources
          else mapped_sources
    in

    List.map
      (fun range -> (range.destination_start, range.length))
      mapped_sources
  in

  let locations =
    List.map
      (fun seed ->
        List.fold_left
          (fun sources map ->
            List.map (fun source -> apply_map source map) sources |> List.concat)
          [ seed ] maps)
      seeds
  in

  List.fold_left
    (fun acc (start, _) -> min acc start)
    max_int (List.concat locations)

let run () =
  let input = read_input "inputs/day_05.txt" in
  let result_one = part_one input in
  let result_two = part_two input in
  print_endline ("Part one: " ^ string_of_int result_one);
  print_endline ("Part two: " ^ string_of_int result_two)
