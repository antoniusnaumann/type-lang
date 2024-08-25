# Type
## A type definition language that transpiles to <your favorite language>
### What is *Type* for?
*Type* is a simple schema definition language primarily for serialization / deserialization which plays nicely with the type system of modern statically typed languages like Swift, Rust, Gleam and others that makes sure. The *Type* build tool generates bindings for all these languages along with the neccessary serialization code. To support an additional language, write a custom generator that implements `LanguageGenerator<YourLanguage>`.

## Alternatives
### GraphQL
- GraphQL schema uses a similar type definition syntax as *Type*
- allows to granularly specify which fields
- server and client need to use and understand the GraphQL query language, whereas *Type* uses JSON as serialization format and relies on serialization/deserialization libraries that are popular in each language ecosystem (e.g. Serde in *Rust*)

### JSON Schema
- uses JSON as serialization format (like *Type*)
- allows cross-language code generation from schema file (like *Type*)
- defines how the JSON data should be formatted, whereas *Type* defines how the deserialized type should look

### Protobuf
<own binary format for payload>
