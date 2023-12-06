defmodule Script do
  def parse_line(line) do
    line
    |> String.split(":")
    |> Enum.at(1)
    |> String.trim()
    |> String.replace(" ", "")
    |> Integer.parse()
    |> elem(0)
  end

  def main do
    input =
      IO.read(:stdio, :all) |> String.split("\n") |> Enum.filter(fn x -> String.length(x) > 0 end)

    time = input
           |> Enum.at(0)
           |> Script.parse_line()
    dist = input
           |> Enum.at(1)
           |> Script.parse_line()

    ans = Enum.map(0..time, fn t -> t * (time-t) end)
          |> Enum.filter(fn d -> d > dist end)
          |> length()

    IO.puts(ans)
  end
end

Script.main()
