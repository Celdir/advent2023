defmodule Script do
  def parse_line(line) do
    line
    |> String.split(":")
    |> Enum.at(1)
    |> String.trim()
    |> String.split(" ")
    |> Enum.filter(fn x -> String.length(x) > 0 end)
    |> Enum.map(fn x -> Integer.parse(x) end)
    |> Enum.map(fn x -> elem(x, 0) end)
  end

  def main do
    input =
      IO.read(:stdio, :all) |> String.split("\n") |> Enum.filter(fn x -> String.length(x) > 0 end)

    times = input
            |> Enum.at(0)
            |> Script.parse_line()
    dists = input
            |> Enum.at(1)
            |> Script.parse_line()

    ans = Enum.zip(times, dists)
          |> Enum.map(fn {time, dist} -> 
            Enum.map(0..time, fn t -> t * (time-t) end)
            |> Enum.filter(fn d -> d > dist end)
            |> length()
          end)
          |> Enum.product()

    IO.puts(ans)
  end
end

Script.main()
