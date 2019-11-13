# Creating relationships

## Implementing `HaveRelationship`

## Using macro `Relationships`

## Notes on data fetching

This library makes no assumptions in terms of how data is fetched. The only 
requirement is that relationships can be provided from the instance of the 
primary data, since `fn get_relation(&self) -> To` only argument is `&self`.

By controlling which fields of the primary data are serialized, it is very much
possible to provide the relationship data however you like:

 - Lazily: Store your database/data fetcher abstraction together with the 
 primary data, don't serialize it, use the abstraction when 
 `fn get_relation(&self) -> To` is called.
 - Eagerly: Fetch and store relationships together with the primary data, don't
 serialize it, return it in `fn get_relation(&self) -> To`.
 
These approaches are illustrated in the following sections.

### Lazy fetching

The following is a basic example showing how we can return an `Author` as a 
relationship when it is fetched when `get_relation` is called:

```rust
use serde::Serialize;

#[derive(Serialize)]
struct Author {
    name: String,
    age: u64,
}

#[derive(Serialize, ResourceType, ResourceIdentifiable)]
struct Book {
    id: u64,
    title: String,
    pages: u64,
    #[serde(skip_serializing)]
    data_fetcher: DataFetcher,
}

impl HaveRelationship<'_, Author> for Book {
    fn get_relation(&self) -> Author {
        &self.data_fetcher.get_author_with_book_id(self.id)
    }
}
```

### Eager fetching

The following is a basic example showing how we can return an `Author` as a 
relationship when it is part of the primary data `Book`:

```rust
use serde::Serialize;

#[derive(Serialize)]
struct Author {
    name: String,
    age: u64,
}

#[derive(Serialize, ResourceType, ResourceIdentifiable)]
struct Book {
    id: u64,
    title: String,
    pages: u64,
    #[serde(skip_serializing)]
    author: Author,
}

impl<'a> HaveRelationship<'a, &Author> for Book {
    fn get_relation(&'a self) -> &'a Author {
        &self.author
    }
}
```

Notice that a reference is returned as a relation. This means we have to 
specify that the reference lives as long as the implementing type.

This approach is simple and elegant in terms of having a sensible definition of
`Book`. It also makes the most sense to fetch `Author` along with `Book`, 
perhaps even in the same query, for efficiency.