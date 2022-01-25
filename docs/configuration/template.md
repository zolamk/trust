---
description: Trust Email & SMS Templating
---

# Templating

{% hint style="info" %}
Type - Object
{% endhint %}

Determines email and sms templating configuration, trust uses [mustache](https://mustache.github.io) template engine

```json
{
    ...
    "change_template": {
        "path": "./change-template.html",
        "sms": "Confirm Your New Phone Number\n{{ phone_change_token }}",
        "subject": "Confirm Your New Email Address"
    }
}
```

### path

{% hint style="info" %}
Type - String
{% endhint %}

path to a template file in mustache format

### subject

{% hint style="info" %}
Type - String
{% endhint %}

email subject

### sms

{% hint style="info" %}
Type - String
{% endhint %}

sms template in mustache format
