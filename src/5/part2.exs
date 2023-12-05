defmodule Script do
  def parse_seeds(line) do
    line
    |> String.split(":")
    |> Enum.at(1)
    |> String.trim()
    |> String.split(" ")
    |> Enum.map(fn x -> Integer.parse(x) end)
    |> Enum.map(fn x -> elem(x, 0) end)
    |> Enum.chunk_every(2)
    |> Enum.map(fn chunk ->
      start = Enum.at(chunk, 0)
      len = Enum.at(chunk, 1)
      {start, len}
    end)
  end

  def parse_range(line) do
    nums = line
           |> String.split(" ")
           |> Enum.map(fn x -> Integer.parse(x) end)
           |> Enum.map(fn x -> elem(x, 0) end)
    {Enum.at(nums, 1), Enum.at(nums, 0), Enum.at(nums, 2)}
  end

  def parse_map(lines) do
    lines
    |> Enum.drop(1)
    |> Enum.map(fn x -> Script.parse_range(x) end)
    |> Enum.sort()
  end

  def intersects?(start1, len1, start2, len2) do
    !(start2 >= start1+len1 || start1 >= start2+len2)
  end

  def intersection(range1, range2) do
    {start1, len1} = range1
    {start2, len2} = range2
    istart = max(start1, start2)
    iend = min(start1+len1, start2+len2)
    ilen = iend - istart
    {istart, ilen}
  end

  def combine_ranges(start, len, ranges) do
    intersections = ranges
                    |> Enum.map(fn {xstart, ystart, rlen} -> 
                      inter = intersection({start, len}, {xstart, rlen})
                      {elem(inter, 0), elem(inter, 1), ystart-xstart}
                    end)
                    |> Enum.filter(fn {_, l, _} -> l > 0 end)
                    |> Enum.sort()

    intersections = [{start, 0, 0}] ++ intersections ++ [{start+len, 0, 0}]

    betweens = Enum.zip(intersections, Enum.drop(intersections, 1))
               |> Enum.map(fn {a, b} -> 
                 {astart, alen, _} = a
                 {bstart, _, _} = b
                 nstart = astart+alen
                 nlen = bstart - nstart
                 {nstart, nlen}
               end)
               |> Enum.filter(fn {_, l} -> l > 0 end)

    intersections = intersections
                    |> Enum.drop(1)
                    |> Enum.drop(-1)

    mapped_inters = intersections
                    |> Enum.map(fn {istart, ilen, delta} -> {istart+delta, ilen} end)

    mapped_inters ++ betweens
  end

  def lookup(map, xrange) do
    {xs, xlen} = xrange
    intersecting = map
                   |> Enum.filter(fn range -> Script.intersects?(xs, xlen, elem(range, 0), elem(range, 2)) end)
    if length(intersecting) > 0 do
      combine_ranges(xs, xlen, intersecting)
    else
      xrange
    end
  end

  def main do
    input =
      IO.read(:stdio, :all) |> String.split("\n") |> Enum.filter(fn x -> String.length(x) > 0 end)

    seeds = input |> Enum.at(0) |> Script.parse_seeds()
    input = input |> Enum.drop(1)

    {seed_to_soil, input} = input |> Enum.split_while(fn line -> String.trim(line) != "soil-to-fertilizer map:" end)
    {soil_to_fert, input} = input |> Enum.split_while(fn line -> String.trim(line) != "fertilizer-to-water map:" end)
    {fert_to_water, input} = input |> Enum.split_while(fn line -> String.trim(line) != "water-to-light map:" end)
    {water_to_light, input} = input |> Enum.split_while(fn line -> String.trim(line) != "light-to-temperature map:" end)
    {light_to_temp, input} = input |> Enum.split_while(fn line -> String.trim(line) != "temperature-to-humidity map:" end)
    {temp_to_humidity, input} = input |> Enum.split_while(fn line -> String.trim(line) != "humidity-to-location map:" end)
    humidity_to_location = input

    seed_to_soil = seed_to_soil |> Script.parse_map()
    soil_to_fert = soil_to_fert |> Script.parse_map()
    fert_to_water = fert_to_water |> Script.parse_map()
    water_to_light = water_to_light |> Script.parse_map()
    light_to_temp = light_to_temp |> Script.parse_map()
    temp_to_humidity = temp_to_humidity |> Script.parse_map()
    humidity_to_location = humidity_to_location |> Script.parse_map()

    locations = seeds
                |> Enum.map(fn seed -> Script.lookup(seed_to_soil, seed) end) |> List.flatten()
                |> Enum.map(fn soil -> Script.lookup(soil_to_fert, soil) end) |> List.flatten()
                |> Enum.map(fn fert -> Script.lookup(fert_to_water, fert) end)  |> List.flatten()
                |> Enum.map(fn water -> Script.lookup(water_to_light, water) end)  |> List.flatten()
                |> Enum.map(fn light -> Script.lookup(light_to_temp, light) end)  |> List.flatten()
                |> Enum.map(fn temp -> Script.lookup(temp_to_humidity, temp) end)  |> List.flatten()
                |> Enum.map(fn humidity -> Script.lookup(humidity_to_location, humidity) end)  |> List.flatten()
    ans = locations
          |> Enum.map(fn range -> elem(range, 0) end)
          |> Enum.min()
    IO.puts(ans)
  end
end

Script.main()
