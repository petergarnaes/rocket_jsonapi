# Resource objects

In the JSON:API specification a resource object can be globally identified by its id __and__ type.
To make a `struct` a resource object, `resourceIdentifiable` must be implemented.

## Implementing`ResourceIdentifiable`

### Interface

### Using `#[derive()]`

#### Customizing `#[derive()]` behaviour
