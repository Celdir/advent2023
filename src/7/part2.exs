defmodule Script do
  def parse_hand(line) do
    parts = line
            |> String.split(" ")
    cards = parts 
            |> Enum.at(0)
            |> String.replace("T", ":")
            |> String.replace("J", "1")
            |> String.replace("Q", "<")
            |> String.replace("K", "=")
            |> String.replace("A", ">")
    bid = parts 
          |> Enum.at(1)
          |> Integer.parse()
          |> elem(0)
    {cards, bid}
  end

  def strength(cards) do
    counts = cards
             |> String.to_charlist()
             |> Enum.frequencies()
             |> Map.to_list()
             |> Enum.map(fn {k, v} -> {v, k} end)
             |> Enum.sort()
             |> Enum.reverse()
    jokers = counts 
             |> Enum.find(fn {_, card} -> card == ?1 end)
    counts = counts
             |> Enum.filter(fn x -> x != jokers end)
    counts = if jokers != nil do
      if length(counts) > 0 do
        first = counts |> Enum.at(0)
        first = {elem(first, 0) + elem(jokers, 0), elem(first, 1)}
        [first] ++ (counts |> Enum.drop(1))
      else
        # All five cards are jokers
        [jokers]
      end
    else
      counts
    end

    first_count = counts 
                  |> Enum.at(0) 
                  |> elem(0)
    case first_count do
      5 -> 7
      4 -> 6
      3 -> case elem(Enum.at(counts, 1), 0) do
        2 -> 5
        _ -> 4
      end
      2 -> case elem(Enum.at(counts, 1), 0) do
        2 -> 3
        _ -> 2
      end
      _ -> 1
    end
  end

  def main do
    input =
      IO.read(:stdio, :all) |> String.split("\n") |> Enum.filter(fn x -> String.length(x) > 0 end)

    hands = input 
            |> Enum.map(fn line -> parse_hand(line) end)
    ranked = hands
             |> Enum.map(fn {cards, bid} -> {strength(cards), cards, bid} end)
             |> Enum.sort()
    ans = ranked 
          |> Enum.zip(1..length(ranked))
          |> Enum.map(fn {{_, _, bid}, rank} -> bid * rank end)
          |> Enum.sum()
    IO.puts(ans)
  end
end

Script.main()
