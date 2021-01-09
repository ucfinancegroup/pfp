# Code Generation Notes

## iOS

For iOS, the `plaid/webhook` route is not needed nor does it [properly compile in Swift](https://github.com/ucfinancegroup/pfp/issues/48).
Before generating Swift code it is recommended to comment out the `/api/plaid/webhook` path until a cleaner alternative is put in place.

After doing any modification to the yaml, the Swift client can be generated as follows:

```
openapi-generator generate -i path/to/api.yml -g swift5 -o path/to/output/folder
```

Additionally, to generate on iOS none of the responses in `api.yaml` can be of type `object` as this translates to `Any` in Swift which is not Codable.
