---
description: Trust SMTP Configuration
---

# SMTP

{% hint style="info" %}
Type - Object
{% endhint %}

Trust email sending configuration

```json
{
    ...
    "smtp": {
        "email": "no-reply@example.com",
        "host": "smtp.example.com",
        "port": "2525",
        "username": "example",
        "password": "example"
    }
}
```

`smtp` has the following properties

### email

{% hint style="info" %}
Type - String

Required
{% endhint %}

SMTP Server email that will be used to send trust emails e.g `no-reply@zelalem.me`

### host

{% hint style="info" %}
Type - String

Required
{% endhint %}

SMTP server host e.g `smtp.zelalem.me`

### port

{% hint style="info" %}
Type - Number

Required
{% endhint %}

SMTP server port e.g `2525`

### username

{% hint style="info" %}
Type - String

Required
{% endhint %}

SMTP server username

### password

{% hint style="info" %}
Type - String

Required
{% endhint %}

SMTP server password