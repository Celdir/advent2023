defmodule Script do
  def parse_seeds(line) do
    line
    |> String.split(":")
    |> Enum.at(1)
    |> String.trim()
    |> String.split(" ")
    |> Enum.map(fn x -> Integer.parse(x) end)
    |> Enum.map(fn x -> elem(x, 0) end)
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

  def lookup(map, x) do
    nearest_range = map
                    |> Enum.filter(fn range -> elem(range, 0) <= x end)
                    |> Enum.at(-1)
    case nearest_range do
      nil -> x
      {xstart, ystart, len} -> 
        if x < xstart+len do
          x + (ystart - xstart)
        else
          x
        end
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
                |> Enum.map(fn seed -> Script.lookup(seed_to_soil, seed) end)
                |> Enum.map(fn soil -> Script.lookup(soil_to_fert, soil) end)
                |> Enum.map(fn fert -> Script.lookup(fert_to_water, fert) end)
                |> Enum.map(fn water -> Script.lookup(water_to_light, water) end)
                |> Enum.map(fn light -> Script.lookup(light_to_temp, light) end)
                |> Enum.map(fn temp -> Script.lookup(temp_to_humidity, temp) end)
                |> Enum.map(fn humidity -> Script.lookup(humidity_to_location, humidity) end)
    ans = locations
          |> Enum.min()
    IO.puts(ans)

  end
end

Script.main()
