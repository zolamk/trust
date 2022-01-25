---
description: Trust Login Hook Configuration
---

# Login Hook

Trust can trigger a hook when a user successfully authenticates or refresh their token, the url for the login hook can be specified at [login_hook](../configuration/README.md#login_hook).

The hook can be used to reject the authentication, reject token refresh or set extra data on the token.

### Hook Authentication

Trust hook calls will contain an `Authorization` header containing a JSON Web Token signed with the configured key, more information on [JWT configuration](../configuration/jwt.md).

#### Sample Authorization Header

```
Authorization: Bearer eyJhbGciOiJFUzUxMiIsInR5cCI6IkpXVCJ9.eyJhdWQiOlsidHJ1c3QiXSwiZXhwIjoxNjQzMTEwNzA3LCJpYXQiOjE2NDMxMTA2NDcsIm1ldGRhdGEiOnsicm9sZXMiOlsidHJ1c3QiXX19.AL6ZYuT8kHvOY3zH975swC4qSCN6-idmyrCOTlYcFc9iZt6qiumMminuqyyqaCEbjLjuuZIpy3Wq-4V_aJUUsa9MAPs9bKmAwtIJDRVVewlUr1fkGHETRw-CBAaR7bfWOrYuG0GKkKc1ZrVk5otriUuA74TbmIsPyt7SSdzmRRqS8490 
```

#### Sample JWT Payload

```json
{
  "aud": [
    "trust"
  ],
  "exp": 1643110707,
  "iat": 1643110647,
  "metdata": {
    "roles": [
      "trust"
    ]
  }
}
```

### Hook Body

Trust hook call will contain a json body containing the following fields

- event - hook event (`login`)
- provider - authentication provider (`password`, `github`, `facebook`, `google`)
- user - user data

#### Example Data Sent To Hook

``` json
{
  "event": "login",
  "provider": "password",
  "user": {
    "id": "OWLLC3",
    "email": "zola@programmer.net",
    "name": "Zelalem Mekonen",
    "email_confirmed": true,
    "email_confirmation_token_sent_at": "2022-01-25T14:35:17.904938+03:00",
    "email_confirmed_at": "2022-01-25T14:36:17.207834+03:00",
    "phone_confirmation_token_sent_at": "2022-01-25T14:35:21.864512+03:00",
    "last_signin_at": "2022-01-25T15:17:29.845687+03:00",
    "created_at": "2022-01-25T14:35:17.895544+03:00",
    "updated_at": "2022-01-25T15:17:26.988077+03:00"
  }
}
```