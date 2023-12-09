defmodule Script do
  def parse_line(line) do
    line
    |> String.trim()
    |> String.split(" ")
    |> Enum.map(fn x -> Integer.parse(x) |> elem(0) end)
  end

  def next_val(sequence) do
    if Enum.all?(sequence, fn x -> x == 0 end) do
      0
    else
      diffs = Enum.zip(sequence, Enum.drop(sequence, 1))
              |> Enum.map(fn {l, r} -> r-l end)
      Enum.at(sequence, -1) + next_val(diffs)
    end
  end

  def main do
    input =
      IO.read(:stdio, :all) |> String.split("\n") |> Enum.filter(fn x -> String.length(x) > 0 end)

    sequences = input 
                |> Enum.map(fn line -> parse_line(line) end)

    ans = sequences
          |> Enum.map(fn seq -> next_val(seq) end)
          |> Enum.sum()
    IO.puts(ans)
  end
end

Script.main()
