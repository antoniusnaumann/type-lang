# Type
## A type definition language that transpiles to <your favorite language>
### What is *Type* for?
*Type* is a simple schema definition language primarily for serialization / deserialization which plays nicely with the type system of modern statically typed languages like Swift, Rust, Gleam and others that makes sure. The *Type* build tool generates bindings for all these languages along with the neccessary serialization code. To support an additional language, write a custom generator that implements `LanguageGenerator<YourLanguage>`.

<you either have the choice between a schema language like json schema that does not translate nicely into most type systems 
or repeating your types and serialization code through different languages

unlike protobuf or graphql, Type is invisible at the API layer, json (<or other supported formats>) are used as transport format.>
