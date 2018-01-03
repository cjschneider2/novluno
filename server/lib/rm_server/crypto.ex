use Bitwise

defmodule RmServer.Crypto do
  @moduledoc false

  def decode(in_data) do
    out_data = decode(in_data, [])
    {:ok, out_data}
  end

  def encode(data) do
    {:ok, data}
  end

  defp decode([], out_data) do out_data end
  defp decode([cur_chr| in_data], out_data) do
    chr = case cur_chr do
      7 ->
        [next_chr| rest_data] = in_data
        bxor(next_chr, 15)
      _ ->
        rest_data = in_data
        cur_chr
    end
    decode(rest_data, out_data ++ [chr])
  end

end
