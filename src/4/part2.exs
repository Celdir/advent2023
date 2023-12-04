defmodule Script do
  def parse_card_part(part) do
    part
    |> String.split(" ")
    |> Enum.map(fn x -> String.trim(x) end)
    |> Enum.filter(fn x -> String.length(x) > 0 end)
    |> Enum.map(fn x -> Integer.parse(x) end)
    |> Enum.map(fn x -> elem(x, 0) end)
  end

  def card_value(line) do
    card = String.split(line, ":") |> Enum.at(1)

    parts =
      card
      |> String.split("|")
      |> Enum.map(fn x -> String.trim(x) end)
      |> Enum.map(fn x -> Script.parse_card_part(x) end)

    winning = parts |> Enum.at(0)
    yours = parts |> Enum.at(1)

    count = MapSet.intersection(MapSet.new(winning), MapSet.new(yours)) |> MapSet.size()
    count
  end

  def main do
    input =
      IO.read(:stdio, :all) |> String.split("\n") |> Enum.filter(fn x -> String.length(x) > 0 end)

    scores = input |> Enum.map(fn x -> Script.card_value(x) end)

    copies =
      scores
      |> Enum.with_index()
      |> Enum.reduce(%{}, fn {score, index}, copies ->
        copies = Map.update(copies, index, 1, fn existing -> existing + 1 end)
        count = Map.get(copies, index)

        if score >= 1 do
          1..score
          |> Enum.map(fn x -> index + x end)
          |> Enum.reduce(copies, fn x, copies ->
            Map.update(copies, x, count, fn existing -> existing + count end)
          end)
        else
          copies
        end
      end)

    sum =
      copies
      |> Map.filter(fn {key, _} -> 0 <= key && key < length(scores) end)
      |> Map.values()
      |> Enum.sum()

    IO.puts(sum)
  end
end

Script.main()
