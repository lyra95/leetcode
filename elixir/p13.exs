defmodule Solution do
  @dict %{"I" => 1, "V" => 5, "X" => 10, "L" => 50, "C" => 100, "D" => 500, "M" => 1000}
  @spec roman_to_int(s :: String.t()) :: integer
  def roman_to_int(s) do
    exe(String.graphemes(s), 0)
  end

  def exe([a, b | t], acc) do
    case [a, b] do
      ["C", "D"] -> exe(t, acc + 400)
      ["C", "M"] -> exe(t, acc + 900)
      ["X", "L"] -> exe(t, acc + 40)
      ["X", "C"] -> exe(t, acc + 90)
      ["I", "V"] -> exe(t, acc + 4)
      ["I", "X"] -> exe(t, acc + 9)
      _ -> exe([b | t], acc + @dict[a])
    end
  end

  def exe([], acc) do
    acc
  end

  def exe([a], acc) do
    acc + @dict[a]
  end
end
