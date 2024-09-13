# KafkaMinimalConsumer

This is a minimal Kafka Consumer that reads metadata and messages from a Kafka topic, using the same implementation
in [RisingWave](https://github.com/risingwavelabs/risingwave)

The tool accepts the following parameters:

| name                           | type                                           | notes                                                                                                                         |
|--------------------------------|------------------------------------------------|-------------------------------------------------------------------------------------------------------------------------------|
| `properties.bootstrap.server`  | required; string                               | NA                                                                                                                            |
| `topic`                        | required; string                               | NA                                                                                                                            |
| `properties.sync.call.timeout` | optional (default to 5s); string               | The String value unit support for one of:`[“y”,“mon”,“w”,“d”,“h”,“m”,“s”, “ms”, “µs”, “ns”]`                                  |
| `group.id`                     | optional(default to `rw-consumer-1-1`); string | [RisingWave](https://github.com/risingwavelabs/risingwave) does not support customize `group.id`, impl here for testing usage |
| `properties.security.protocol` | optional; string                               | refer to vendor settings                                                                                                      |
| `properties.sasl.mechanism`    | optional; string                               | refer to vendor settings                                                                                                      |
| `properties.sasl.username`     | conditional; string                            | refer to vendor settings                                                                                                      |
| `properties.sasl.password`     | conditional; string                            | refer to vendor settings                                                                                                      |

The params should be provided in JSON format, for example:

```bash
./kafkaminimal \
"{
  \"properties.bootstrap.server\": \"localhost:9092\",
  \"topic\": \"test\",
  \"properties.sync.call.timeout\": \"5s\",
  \"group.id\": \"rw-consumer-1-1\",
  \"properties.security.protocol\": \"SASL_SSL\",
  \"properties.sasl.mechanism\": \"PLAIN\",
  \"properties.sasl.username\": \"foo_user\",
  \"properties.sasl.password\": \"bar_password\"}"
```
