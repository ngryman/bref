# bref

![](banner.png)

> A modest URL shortener for personal usage.

Shortens URLs to a 6 characters-length key that is generated based on current time. Why?
[Why not](src/key.rs)?

## Usage

### Shorten a URL

```
$ http post :8080 url="https://ngryman.sh"
HTTP/1.1 200 OK
content-length: 40
content-type: application/json
date: Sat, 07 Oct 2023 13:16:43 GMT

{
    "key": "1qp7Ah",
    "url": "https://ngryman.sh"
}
```

### Redirect

```
$ http get :8080/1qp6Oj
HTTP/1.1 307 Temporary Redirect
content-length: 0
date: Sat, 07 Oct 2023 13:17:43 GMT
location: https://ngryman.sh
```

## Deployment

Bref can easily be deployed to [fly.io](https://fly.io). Please refer to the
[deploy](.github/workflows/deploy.yaml) workflow pipeline.
