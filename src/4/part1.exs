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

    if count == 0 do
      0
    else
      2 ** (count - 1)
    end
  end

  def main do
    input =
      IO.read(:stdio, :all) |> String.split("\n") |> Enum.filter(fn x -> String.length(x) > 0 end)

    scores = input |> Enum.map(fn x -> Script.card_value(x) end)
    ans = scores |> Enum.sum()
    IO.puts(ans)
  end
end

Script.main()
