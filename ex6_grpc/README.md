
https://github.com/hyperium/tonic/blob/master/examples/helloworld-tutorial.md

```shell
# サーバー起動
cargo run --bin helloworld-server
# grpcurlクライアントによる接続
grpcurl -plaintext -import-path ./proto -proto helloworld.proto -d '{"name": "Tonic"}' '[::1]:50051' helloworld.Greeter/SayHello
```