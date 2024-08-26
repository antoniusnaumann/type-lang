import gleam/decode
import gleam/option.{type Option}
import types/account.{type Account}

pub type User {
  User(
    name: String,
    nickname: Option(String),
    level: Int,
    is_admin: Bool,
    account: Account,
  )
}

pub fn decode(data: Dynamic) {
  let decoder =
    decode.into({
      use name <- decode.parameter
      use nickname <- decode.parameter
      use level <- decode.parameter
      use is_admin <- decode.parameter
      use account <- decode.parameter

      User(name, nickname, level, is_admin, account)
    })
    |> decode.field("name", decode.string)
    |> decode.field("nickname", decode.optional(decode.string))
    |> decode.field("level", decode.int)
    |> decode.field("is_admin", decode.bool)
    |> decode.field("account", account.decode)

  decoder |> decode.from(data)
}
