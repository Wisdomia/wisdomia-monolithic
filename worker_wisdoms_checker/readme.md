# Adding new wisdom

1. Update wisdoms.json file.

Example:

#### Previous

```typescript
[
    {"description": "Every day, every minute, every second, we are creating evidence for our self that we are qualified for the life we want!"},
    {"description":"Hope it works!"},
    {"description": "Knowledge is power."},
    {"description": "Time is money."}
]
```

#### New


```typescript
[
    {"description": "Every day, every minute, every second, we are creating evidence for our self that we are qualified for the life we want!"},
    {"description":"Hope it works!"},
    {"description": "Knowledge is power."},
    {"description": "Time is money."},
    {"description": "New Wisdom"}
]
```

2. Run command

```shell
base64 wisdoms.json
```

Result should be something like this:

```
WwogICAgeyJkZXNjcmlwdGlvbiI6ICJFdmVyeSBkYXksIGV2ZXJ5IG1pbnV0ZSwgZXZlcnkgc2Vj
b25kLCB3ZSBhcmUgY3JlYXRpdwdqdwdqddqdqwdqdqwd...ciBzZWxmIHRoYXQgd2UgYXJlIHF1
ICAgeyJkZXNjcmlwdGlvbiI6IkFBQUFBQUFBQUFBQUFBQUFBIn0sCiAgICB7ImRlc2NyaXB0aW9u
IjoiQkJCQkJCQkJCQkJCQkJCQkJCQiJ9Cl0KICA=
```

3. Copy the result


4. Paste the result into file `encoded-wisdoms.b64`

5. If the `worker-wisdoms-checker` is running, it will pick up the changes and update the database.