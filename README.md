# QRCode

QR Code encoder in Rust.

## Installation

If [available in Hex](https://hex.pm/docs/publish), the package can be installed
by adding `qr_code` to your list of dependencies in `mix.exs`:

```elixir
def deps do
  [{:qr_code, "~> 0.1.0"}]
end
```

Documentation can be generated with [ExDoc](https://github.com/elixir-lang/ex_doc)
and published on [HexDocs](https://hexdocs.pm). Once published, the docs can
be found at [https://hexdocs.pm/qr_code](https://hexdocs.pm/qr_code).

## Usage

```elixir
iex> QRCode.generate_string("12345")
...> {:ok, string}
```

```elixir
iex> QRCode.generate_svg("12345")
...> {:ok, svg_string}
```

