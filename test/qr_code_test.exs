defmodule QRCodeTest do
  use ExUnit.Case
  doctest QRCode

  test "the truth" do
    assert 1 + 1 == 2
  end

  describe "generate_svg/1" do
    test "generate an svg" do
      {:ok, svg} = QRCode.generate_svg("weixin://wxpay/bizpayurl?pr=2t9fNJM")

      assert svg
    end
  end

  describe "generate_string/1" do
    test "generate an string" do
      {:ok, string} = QRCode.generate_string("weixin://wxpay/bizpayurl?pr=2t9fNJM")

      assert string
    end
  end
end
