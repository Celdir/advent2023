defmodule Script do
  def parse_children(line) do
    parts = line
            |> String.split("=")
    node = parts 
           |> Enum.at(0)
           |> String.trim()
    lr = parts 
         |> Enum.at(1)
         |> String.trim()
         |> String.replace("(", "")
         |> String.replace(")", "")
         |> String.split(",")
    left = lr
           |> Enum.at(0)
           |> String.trim()
    right = lr
            |> Enum.at(1)
            |> String.trim()
    {node, left, right}
  end

  def dist_to_z(tree, instructions, node) do
    traverse = instructions 
               |> String.graphemes()
               |> Stream.cycle()
               |> Enum.reduce_while({node, 0}, fn instr, {node, steps} ->
                 if String.ends_with?(node, "Z") do
                   {:halt, {node, steps}}
                 else
                   {left, right} = Map.get(tree, node)
                   if instr == "L" do
                     {:cont, {left, steps+1}}
                   else
                     {:cont, {right, steps+1}}
                   end
                 end
               end)
    traverse |> elem(1)
  end

  def lcm(a, b) do
    div(a * b, Integer.gcd(a, b))
  end 

  def main do
    input =
      IO.read(:stdio, :all) |> String.split("\n") |> Enum.filter(fn x -> String.length(x) > 0 end)

    instructions = input |> Enum.at(0)
    input = input |> Enum.drop(1)

    tree = input 
           |> Enum.map(fn line -> parse_children(line) end)
           |> Enum.reduce(%{}, fn {node, left, right}, map -> Map.put(map, node, {left, right}) end)
    starting = tree
               |> Map.keys()
               |> Enum.filter(fn node -> String.ends_with?(node, "A") end)
    dists = starting 
            |> Enum.map(fn node -> dist_to_z(tree, instructions, node) end)
    ans = dists
          |> Enum.reduce(1, fn dist, acc -> lcm(acc, dist) end)
    IO.puts(ans)
  end
end

Script.main()
