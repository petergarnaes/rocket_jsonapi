# JSON:API requests

TODO Remember to note the Gotcha of status 406 and 415, ie. when params are used in Content-Type and Accept HTTP 
headers. Specification states nothing other than the status codes, so it must be shown how a user would implement a 
[catcher](https://api.rocket.rs/v0.4/rocket/struct.Catcher.html) and return valid JSON:API with their own error message
if the wish.