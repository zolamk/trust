---
description: Trust Configuration Documentation
---

# Configuration

Trust is configurable using a json file, a minimal json configuration file will look something like the following

```json
{
    "disable_phone": true,
    "site_url": "http://localhost:3002",
    "instance_url": "http://localhost:8082",
    "database_url": "postgres://postgres:password@localhost:5432/trust",
    "smtp": {
        "host": "smtp.mailtrap.io",
        "port": 2525,
        "username": "username",
        "password": "password",
        "email": "no-reply@zelalem.me"
    },
    "jwt": {
        "algorithm": "HS256",
        "secret": "super_duper_tipper_secret"
    }
}
```

## Field List

### access\_token\_cookie\_name

{% hint style="info" %}
Type - String

Default - **trust\_access\_token**
{% endhint %}

Defines what name the access token cookie will use, the access token cookie will be an `httponly` and `secure` cookie

### access\_token\_cookie\_domain

{% hint style="info" %}
Type - String
{% endhint %}

Defines which domain the access token will be sent to by the browser, specifically sets the `Domain` attribute of the cookie

> The `Domain` attribute specifies which hosts can receive a cookie. If unspecified, the attribute defaults to the same host that set the cookie, _excluding subdomains_. If `Domain` _is_ specified, then subdomains are always included. Therefore, specifying `Domain` is less restrictive than omitting it.
>
> [https://developer.mozilla.org/en-US/docs/Web/HTTP/Cookies#define\_where\_cookies\_are\_sent](https://developer.mozilla.org/en-US/docs/Web/HTTP/Cookies#define\_where\_cookies\_are\_sent)
> 

### admin\_only\_list

{% hint style="warning" %}
Type - Boolean

Default - **true**
{% endhint %}

Determines whether non admin users can list users specifically whether non admin users can run the `users` and `user` graphql queries, setting this option to `false` will allow normal users to list user data

### admin\_roles

{% hint style="hint" %}
Type - Array

Default - ["trust:admin"]

{% endhint %}

Determines which roles are allowed to perform admin actions.

### change\_template

{% hint style="info" %}
Type - Object
{% endhint %}

Determines templates to be used when sending email address change email and phone number change sms, more on [template configuration](template.md)

trust uses the following template for emails by default if [path](template.md#path)  is not provided

```html
<h2>Change Your Email Address</h2>
<p>Follow this link to confirm your email address change</p>
<p>
    <a href='{{ site_url }}?token={{ email_change_token }}'>Confirm</a>
</p>
```

with [subject](template.md#subject) as **Confirm Email Change**

email change template will be passed the following context `site_url`, `email_change_token`, `new_email`, `instance_url`

trust uses the following template for phone number change sms by default if [sms](template.md#sms) is not provided

```
Phone change code -  {{ phone_change_token }}
```

phone change template will be passed `site_url`, `phone_change_token`, `new_phone`, `instance_url`

### **confirmation\_template**

{% hint style="info" %}
Type - Object
{% endhint %}

Determines templates to be used when sending email address confirmation email and phone number confirmation sms, more on [template configuration](template.md)

trust uses the following template for emails by default if [path](template.md#path) is not provided

```html
<h2>Confirm Your Email Address</h2>
<p>Follow this link to confirm your email</p>
<p>
    <a href='{{ site_url }}?token={{ email_confirmation_token }}'>Confirm</a>
</p>
```

with [subject](template.md#subject) as **Confirm Your Account**

email confirmation template will be passed the following context `site_url`, `email_confirmation_token`, `instance_url`

trust uses the following template for confirmation sms by default if [sms](template.md#sms) is not provided

```
Phone confirmation code - {{ phone_confirmation_token }}
```

phone confirmation template will be passed `site_url`, `phone_confirmation_token`, `instance_url`

### custom_data_schema

{% hint style="info" %}
Type - Object
{% endhint %}

trust supports storing and querying custom data, you can specify the schema of your custom data using the `custom_data_schema` field in your configuration file, more information on [custom data](./custom-data.md)

### database\_url

{% hint style="info" %}
Type - String

Required
{% endhint %}

Postgres database connection url e.g `postgresql://username:password@host:5432/database`

### disable\_email

{% hint style="info" %}
Type - Boolean

Default - **false**
{% endhint %}

Determines whether email signup and authentication is enabled

### disable\_phone

{% hint style="info" %}
Type - Boolean

Default - **false**
{% endhint %}

Determines whether phone signup and authentication is enabled

### disable\_signup

{% hint style="info" %}
Type - Boolean

Default - **false**
{% endhint %}

Disables user signup, users can only be created by admins

### email\_rule

{% hint style="info" %}
Type - String

Default - **`^[\w-.]+@([\w-]+.)+[\w-]{1,}$`**
{% endhint %}

Determines what format will be accepted by trust as a valid email address, value should be a valid [Golang regular expression](https://github.com/google/re2/wiki/Syntax)

### facebook

{% hint style="info" %}
Type - Object
{% endhint %}

Configures facebook social authentication options, more details on [facebook](social-authentication.md), facebook social authentication is disabled by default.

### github

{% hint style="info" %}
Type - Object
{% endhint %}

Configures github social authentication options, more details on [github](social-authentication.md), github social authentication is disabled by default.

a github app id and secret can be obtained by creating a [github oauth app](https://docs.github.com/en/developers/apps/building-oauth-apps/creating-an-oauth-app).

### google

{% hint style="info" %}
Type - Object
{% endhint %}

Configures google social authentication options, more details on [google](social-authentication.md), google social authentication is disabled by default.

### host

{% hint style="info" %}
Type - String

Default - **localhost**
{% endhint %}

The hostname trust should bind to

### instance\_url

{% hint style="info" %}
Type - String

Required
{% endhint %}

The url where trust will be accessible at, this will be used when building redirect uri for social authentication

### invitation\_template

{% hint style="info" %}
Type - Object
{% endhint %}

Determines templates to be used when sending invitation email and phone invitation sms, more on [template configuration](template.md)

trust uses the following template for invitation emails by default if [path](template.md#path)  is not provided

```html
<h2>You Have Been invited</h2>
<p>Follow this link to accept your invitation</p>
<p>
    <a href='{{ site_url }}?token={{ email_invitation_token }}'>Accept Invite</a>
</p>
```

with [subject](template.md#subject) as **You've Been Invited**

email invitation template will be passed the following context `site_url`, `email_invitation_token`, `instance_url`

trust uses the following template for invitation sms by default if [sms](template.md#sms) is not provided

```
Phone Invitation Code - {{ phone_invitation_token }}
```

phone invitation template will be passed `site_url`, `phone_invitation_token`, `instance_url`

### jwt

{% hint style="info" %}
Type - Object

Required
{% endhint %}

Configures [jwt](https://jwt.io/introduction) options, more details on the [jwt field](jwt.md)

### lockout\_policy

{% hint style="info" %}
Type - Object
{% endhint %}

Configures account lockout policy for incorrect password authentication attempts, more details on the [lockout_policy](lockout-policy.md)

### login\_hook

{% hint style="info" %}
Type - String
{% endhint %}

Hook to be called when the user is authenticating or refreshing their token

### log\_level

{% hint style="info" %}
Type - String

Default - **info**
{% endhint %}

### max\_connection\_pool\_size

{% hint style="info" %}
Type - Number

Default - **10**
{% endhint %}

Determines PostgreSQL connection pool size

### metadata\_path

{% hint style="info" %}
Type - String

Available From - `v0.4.0`
{% endhint %}

When trust triggers the login hook set whatever response the hook returns is set as `metadata` claim on the JWT generated, the `metadata_path` configuration will allow you to pick the data to be used using [JSONPath](https://goessner.net/articles/JsonPath/)

for example assuming your login hook returns data such as below

```json
{
    "id": "user_id",
    "user": {
        "roles": ["role_1", "role_2", "role_3"]
    }
}
```
if you were to set `metadata_path` to `$.user` trust will set the `metadata` claim as

```json
{
    "roles": ["role_1", "role_2", "role_3"]
}
```
more information on [JSONPath](https://goessner.net/articles/JsonPath/)


### minutes\_between\_email\_change

{% hint style="info" %}
Type - Number

Default - **1440**
{% endhint %}

Determines how often a user should be able to change their email address in minutes

### minutes\_between\_phone\_change

{% hint style="info" %}
Type - Number

Default - **1440**
{% endhint %}

Determines how often a user will be able to change their phone number in minutes by default a user will only be to change their phone number every 24 hours or 1440 minutes

### minutes\_between\_resend

{% hint style="info" %}
Type - Number

Default - **10**
{% endhint %}

Determines how often a user will be able to resend confirmation email and sms, by default a user will only be to resend confirmation sms and email every 10 minutes

### password\_hash\_cost

{% hint style="info" %}
Type - Number

Default - 10
{% endhint %}

Trust uses [bcrypt](https://en.wikipedia.org/wiki/Bcrypt) to hash passwords for storage in the database, `password_hash_cost` determines the hash cost

### password\_rule

{% hint style="info" %}
Type - String

Default - **`.{8,1000}`**
{% endhint %}

Determines what password format will be accepted by trust as a valid password, value should be a valid [Golang regular expression](https://github.com/google/re2/wiki/Syntax)

### phone\_rule

{% hint style="info" %}
Type - String

Default - **+\d{5,15}**
{% endhint %}

Determines what phone number format will be accepted by trust as a valid phone number, value should be a valid [Golang regular expression](https://github.com/google/re2/wiki/Syntax)

### port

{% hint style="info" %}
Type - String

Default - **1995**
{% endhint %}

Determines what port trust will bind to

### read\_only\_roles

{% hint style="info" %}
Type - Array

Default - ["trust:read"]

{% endhint %}

Determines read only roles, specifically if a jwt generated for user contains any of the roles specified they will be able to read user data

### recovery\_template

{% hint style="info" %}
Type - Object
{% endhint %}

Determines templates to be used when sending account recovery email and account recovery sms, more on [template configuration](template.md)

trust uses the following template for recovery emails by default if [path](template.md#path) is not provided

```html
<h2>Recover Your Account</h2>
<p>Follow this link to recover you account</p>
<p>
    <a href='{{ site_url }}?token={{ email_recovery_token }}'>Recover</a>
</p>
```

with [subject](template.md#subject) as **Recover Your Account**

email recovery template will be passed the following context `site_url`, `email_recovery_token`, `instance_url`

trust uses the following template for account recovery sms by default if [sms](template.md#sms) is not provided

```
Phone Recovery Code - {{ phone_recovery_token }}
```

Account recovery sms template will be passed `site_url`, `phone_recovery_token`, `instance_url`

### refresh\_token\_cookie\_name

{% hint style="info" %}
Type - String

Default - **trust\_refresh\_token**
{% endhint %}

determines the name of the refresh token cookie

### refresh\_token\_cookie\_domain**

{% hint style="info" %}
Type - String

{% endhint %}

Defines which domain the refresh token will be sent to by the browser, specifically sets the `Domain` attribute of the cookie

> The `Domain` attribute specifies which hosts can receive a cookie. If unspecified, the attribute defaults to the same host that set the cookie, _excluding subdomains_. If `Domain` _is_ specified, then subdomains are always included. Therefore, specifying `Domain` is less restrictive than omitting it.
>
> [https://developer.mozilla.org/en-US/docs/Web/HTTP/Cookies#define\_where\_cookies\_are\_sent](https://developer.mozilla.org/en-US/docs/Web/HTTP/Cookies#define\_where\_cookies\_are\_sent)
> 
### set\_cookies

{% hint style="info" %}
Type - string

Default - **true**

{% endhint %}

determines if access token and refresh token cookies should be set

### site\_url

{% hint style="info" %}
Type - String

Required - true
{% endhint %}

`site_url` is used when forming redirect urls and is passed to email and sms templates

### sms

{% hint style="info" %}
Type - Object
{% endhint %}

configures sms sending options, more details on the [sms](sms.md)

### smtp

{% hint style="info" %}
Type - Object
{% endhint %}

configures email sending options, more details on the [smtp](smtp.md)

### social\_redirect\_page

{% hint style="info" %}
Type - String

Default - **social**
{% endhint %}

configures redirect page for social authentication, see also [instance\_url](./#instance\_url)
