defmodule QRCode do
  @moduledoc """
  Documentation for QRCode.
  """

  use Rustler, otp_app: :qr_code, crate: :qr_code

  # When loading a NIF module, dummy clauses for all NIF function are required.
  # NIF dummies usually just error out when called when the NIF is not loaded, as that should never normally happen.
  def generate_svg(_arg1), do: exit(:nif_not_loaded)
  def generate_string(_arg1), do: exit(:nif_not_loaded)
end
