---
description: Trust SMS Sending Configuration
---

# SMS

{% hint style="info" %}
Type - Object

Required
{% endhint %}

Determines configuration to be used when sending sms

```json
{
    ...
    "sms": {
        "url": "https://api.example.com/send_sms",
        "method": "POST",
        "source": "TRUST",
        "mapping": {
            "source": "from",
            "destination": "to",
            "message": "text"
        },
        "headers": {
            "Authorization": "Bearer super_secret_api_key"
        },
        "extra": {
            "app": "trust"
        }
    }
}
```

### url

{% hint style="info" %}
Type - String

Required
{% endhint %}

URL to be used to send sms e.g

### method

{% hint style="info" %}
Type - String

Required
{% endhint %}

HTTP method to be used when calling the sms sending url

### source

{% hint style="info" %}
Type - String

Required
{% endhint %}

this parameter should be the sender id you wish to use, it could be a phone number, a short code or an alphanumeric string.

### mapping

{% hint style="info" %}
Type - Object

Required
{% endhint %}

`mapping` is used when creating the request body to be sent to the sms api url, the request body will be sent as a json object, `mapping` will be used to map the `source`, `destination`, `message` to what the sms sending api expects, the following examples are for some of the SMS API providers

#### Plivo

```json
{
    ...
    "sms": {
        "mapping":{
            "source": "src",
            "destination": "dst",
            "message": "text"
        }
    }
}
```

#### Nexmo

```json
{
    ...
    "sms": {
        ...
        "mapping":{
            "source": "from",
            "destination": "to",
            "message": "text"
        }
    }
}
```

### headers

{% hint style="info" %}
Type - Object

{% endhint %}

headers to be set on the request to send an sms e.g

``` json
{
    ...
    "sms": {
        ...
        "headers": {
            "Authorization": "Bearer super_secret_api_key"
        }
    }
}
```

### extra

{% hint style="info" %}
Type - Object

{% endhint %}

extra data to be set on the request body, include here any extra data you want appended to the body e.g

```json
{
  ...
  "sms": {
      ...
      "extra": {
          "channel": "sms",
          "message_type": "text"
      }
  }
}
```

in the above example trust will append `channel` and `message_type` to their respective values on the request body