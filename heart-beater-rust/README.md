# heart-beater-rust

```yaml
http:
  - target_url: https://example.com
    cron: "0/5 * * * * *"
    heartbeat_url: https://heatbeat.com
    status:
      - 200
s3:
  - region: ap-northeast-1
    bucket: some-bucket
    prefix: "hoge/"
    cron: "0 * * * * *"
    grace: 1 hour
    heartbeat_url: https://heartbeat.com
    min_size: 100K
```
