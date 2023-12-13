defmodule Script do
  def smudge_reflects?(pattern, index) do
    {first, second} = if index < div(length(pattern), 2) do
      first = pattern |> Enum.take(index+1)
      second = pattern |> Enum.slice(index+1, index+1)
      {first, second}
    else
      remain = length(pattern) - (index+1)
      second = pattern |> Enum.take(-1 * remain)
      first = pattern |> Enum.slice(index-remain+1, remain)
      {first, second}
    end
    dist = Enum.zip(first, Enum.reverse(second))
    |> Enum.map(fn {a, b} -> distance(a, b) end)
    |> Enum.sum()
    dist == 1
  end

  def distance(s, t) do
    sg = s |> String.graphemes()
    tg = t |> String.graphemes()
    Enum.zip(sg, tg)
    |> Enum.map(fn {sc, tc} ->
      if sc == tc do 0 else 1 end
    end)
    |> Enum.sum()
  end

  def transpose(pattern) do
    pattern
    |> Enum.map(fn line -> String.graphemes(line) end)
    |> Enum.zip()
    |> Enum.map(fn t -> Tuple.to_list(t) end)
    |> Enum.map(fn l -> Enum.join(l) end)
  end

  def score(pattern) do
    (0..length(pattern)-2)
    |> Enum.filter(fn index -> smudge_reflects?(pattern, index) end)
    |> Enum.map(fn index -> index+1 end)
    |> Enum.sum()
  end

  def main do
    input =
      IO.read(:stdio, :all) |> String.split("\n")

    patterns = input
               |> Enum.chunk_while([],
                 fn line, acc ->
                   if String.length(line) == 0 do
                     {:cont, acc, []}
                   else
                     {:cont, acc ++ [line]}
                   end
                 end,
                 fn acc ->
                   if length(acc) == 0 do
                     {:cont, []}
                   else
                     {:cont, acc, []}
                   end
                 end)
    horizontal = patterns
                 |> Enum.map(fn pattern -> score(pattern) end)
                 |> Enum.sum()
    vertical = patterns
               |> Enum.map(fn pattern -> transpose(pattern) end)
               |> Enum.map(fn pattern -> score(pattern) end)
               |> Enum.sum()
    ans = 100*horizontal + vertical
    IO.puts(ans)
  end
end

Script.main()
