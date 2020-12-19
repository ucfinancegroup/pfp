# Code Generation Notes

## iOS

For iOS, the `plaid/webhook` route is not needed nor does it [properly compile in Swift](https://github.com/ucfinancegroup/pfp/issues/48).
Before generating Swift code it is recommended to comment out the `/api/plaid/webhook` path until a cleaner alternative is put in place.
