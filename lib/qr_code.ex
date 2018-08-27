defmodule QRCode do
  @moduledoc """
  QRCode encoder
  """

  use Rustler, otp_app: :qr_code, crate: :qr_code

  @doc """
  Generate SVG QRCode

  ## Example
      iex> QRCode.generate_svg("12345")
      {:ok, "<svg>...</svg>"}
  """
  @spec generate_svg(String.t()) :: {:ok, String.t()}
  def generate_svg(_data), do: exit(:nif_not_loaded)

  @doc """
  Generate String QRCode

  ## Example
      iex> QRCode.generate_string("12345")
      {:ok, "###\\n # \\n## \\n"}
  """
  @spec generate_string(String.t()) :: {:ok, String.t()}
  def generate_string(_data), do: exit(:nif_not_loaded)
end
