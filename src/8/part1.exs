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

  def main do
    input =
      IO.read(:stdio, :all) |> String.split("\n") |> Enum.filter(fn x -> String.length(x) > 0 end)

    instructions = input |> Enum.at(0)
    input = input |> Enum.drop(1)

    tree = input 
           |> Enum.map(fn line -> parse_children(line) end)
           |> Enum.reduce(%{}, fn {node, left, right}, map -> Map.put(map, node, {left, right}) end)
    traverse = instructions 
               |> String.graphemes()
               |> Stream.cycle()
               |> Enum.reduce_while({"AAA", 0}, fn instr, {node, steps} ->
                 if node == "ZZZ" do
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
    ans = traverse |> elem(1)
    IO.puts(ans)
  end
end

Script.main()
