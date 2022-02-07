---
description: Trust Custom Data Documentation
---

# Custom Data

{% hint style="info" %}
Type - Object
{% endhint %}

```json
{
    ...
    "custom_data_schema": {
        "age": {
            "type": "integer",
            "minimum": 18,
            "maximum": 30,
            "required": true
        },
        "gender": {
            "type": "string",
            "required": true,
            "choices": [
                "male",
                "female"
            ]
        },
        "subscribe_to_newsletter": {
            "type": "boolean",
            "required": false
        },
        "id": {
            "type": "string",
            "required": true,
            "format": "ID\\d{5,10}"
        }
    }
}
```

`custom_data_schema` value should be an object of [field specifications](./custom-data.md/#field), if `custom_data_schema` is not specified trust will reject all custom data

## Field

{% hint type="info" %}
Type - Object
{% endhint %}

contains the actual field specification used to validate custom data

### type

{% hint type="info" %}

Type - String

**Required**

Options - `string`, `integer`, `float`, `boolean`

{% endhint %}

determines the fields type

### required

{% hint type="info" %}
Type - Boolean

Default - **false**

{% endhint %}

determines whether the field is required or not

### minimum

{% hint type="info" %}

Type - Float

{% endhint %}

determines the minimum amount a field can contain, only applies for `string`, `integer` and `float` field types.

if the field type is `string` minimum is used to enforce the minimum character length.

### maximum

{% hint type="info" %}

Type - Float

{% endhint %}

determines the maximum amount a field can contain, only applies for `string`, `integer` and `float` field types.

if the field type is `string` maximum is used to enforce the maximum character length.

### format

{% hint type="info" %}
Type - String
{% endhint %}

determines the format a field value must take, only applies for `string` field type.

`format` value should be a regular expression in a valid [Golang regular expression](https://github.com/google/re2/wiki/Syntax) format

### choices

{% hint type="info" %}
Type - Array of String
{% endhint %}

determines the valid options for a `string` field type.