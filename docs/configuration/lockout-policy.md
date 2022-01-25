---
description: Trust Lockout Policy Configuration
---

# Lockout Policy

{% hint style="info" %}
Type - Object
{% endhint %}

`lockout_policy` determines what happens during a repeated incorrect password login attempt

```json
{
    ...
    "lockout_policy": {
        "attempts": 5,
        "for": 60
    }
}
```

`lockout_policy` has the following properties

### attempts

{% hint style="info" %}
Type - Number

Default - **5**
{% endhint %}

`attempts` determines how many tries a user has before their account is locked, by default a user has 5 attempts.

### for

{% hint style="info" %}
Type - Number

Default - **60**
{% endhint %}

`for` determines how long a users account will be locked in minutes after they have finished their determined number of attempts, by default a users account will be locked for 60 minutes.