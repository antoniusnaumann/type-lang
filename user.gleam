import gleam/decode
import gleam/option.{type Option}
import gleam/dict.{type Dict}
import types/armorkind.{type ArmorKind}
import types/account.{type Account}
import types/item.{type Item}
import types/tag.{type Tag}

pub type User {
	User(name: String, nickname: Option(String), level: Int, is_admin: Bool, account: Account, tags: List(Tag), armor: Map(ArmorKind, Item))
}

pub fn decode(data: Dynamic) {
	let decoder = decode.into({
		use name <- decode.parameter
		use nickname <- decode.parameter
		use level <- decode.parameter
		use is_admin <- decode.parameter
		use account <- decode.parameter
		use tags <- decode.parameter
		use armor <- decode.parameter

		User(name, nickname, level, is_admin, account, tags, armor)
	})
	|> decode.field("name", decode.string)
	|> decode.field("nickname", decode.optional(decode.string))
	|> decode.field("level", decode.int)
	|> decode.field("is_admin", decode.bool)
	|> decode.field("account", account.decode)
	|> decode.field("tags", decode.list(tag.decode))
	|> decode.field("armor", decode.dict(armorkind.decode, item.decode))

	decoder |> decode.from(data)
}
