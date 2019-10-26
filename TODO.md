# TODO

## Crate stuff

 - Dependencies on syn/quote in derive crate, should it use the same?
 - macro hygiene

## API stuff

 - [x] Overhaul module structure, it is getting messy in `response.rs` and `data.rs`, new modules probably needed
 - Handle top level `errors` parsing properly, should be able to handle multiple errors and parse the top level key
  `errros`
 - Remove all warnings in `rocket_jsonapi` crate, lots of unused stuff.
 - [x] Disable unused warnings in `test_suite` crate.
 - Remove `DataObject` and any other types that are constructed when serializing, we don't want the overhead!
 - [x] Fix deriving of `ResourceIdentifiable`! Should be able to handle `IdType` now.
 - [x] Deriving `ResourceIdentifiable` should handle when `IdType=&str`, can copying be avoided?
 - [x] Write serialization tests of all the newtypes: `ResourceIdentifiableWrapper`, `JsonApiPrimaryDataObject` and
  `JsonApiPrimaryDataObjectArray`
 - [x] Hide `ResourceIdentifier` from user, move to core
 - Make `ResourceIdentifierWrapper` with serialization implementation, so when constructing resource identifiers, we
  can simply read the objects we convert, instead of constructing new.
 - Write full-stack'ish serialization tests for `JsonApiResponse` with all sorts of implementations for the wrapped
  type.
 - Expand `Linkify` derivable API, so static links, relationships etc. can be included
 - Make `Relationships` derivable, consider its current API
 - Make the `Included` API, probably use same approach as relationships API
 - Make all our traits derivable with newtypes, so inheritance boilerplate can be reduced. This pattern could make
  for some nifty implementations with heavy re-use of code through inheritance, only overriding the parts the user
   wishes. For example, a `Person` could also be used as a relationship, like `Author`, but where `links` are changed
   , or maybe entirely excluded.
 - Implement `JsonApi` as a [request guard](https://rocket.rs/v0.4/guide/requests/#custom-guards), see rockets `Json
 ` as [reference](https://github.com/SergioBenitez/Rocket/blob/master/contrib/lib/src/json.rs).
 - Implement `JsonApi` as a [responser](https://rocket.rs/v0.4/guide/responses/#custom-responders), see rockets `Json
 ` as [reference](https://github.com/SergioBenitez/Rocket/blob/master/contrib/lib/src/json.rs).
 - Tests for `JsonApi`
 - Probably a ton more, that I forgot...
 
## Documentation stuff

 - All of it :(