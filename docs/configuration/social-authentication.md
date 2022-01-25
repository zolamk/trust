---
description: Trust Social Registration & Authentication Configuration
---

# Social Authentication

{% hint style="info" %}
Type - Object
{% endhint %}

Trust social registration & authentication supports the following providers so far, `google`, `github` and `facebook`

```json
{
    ...
    "google": {
        "enabled": true
        "id": "google_client_id",
        "secret": "google_client_secret"
    },
    "github": {
        "enabled": true,
        "id": "github_client_id",
        "secret": "github_client_secret"
    },
    "facebook": {
        "enabled": true,
        "id": "facebook_client_id",
        "secret": "facebook_client_secret"
    }
}
```

### enabled

{% hint style="info" %}
Type - Boolean

Default - **false**
{% endhint %}

Determines whether a social provider is enabled or not

### id

{% hint style="info" %}
Type - String

**Conditionally Required**
{% endhint %}

social providers client id, `id` is required only if the social provider is enabled

### secret

{% hint style="info" %}
Type - String

**Conditionally Required**
{% endhint %}

social providers client secret, `secret` is required only if the social provider is enabled
