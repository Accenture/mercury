# Topic Substitution

Some enterprises do not allow automatic creation of messaging topics by user applications.

In this case, you can turn on topic substitution with the following parameters in application.properties

```
application.feature.topic.substitution=true
topic.substitution.file=file:/tmp/config/topic-substitution.yaml,classpath:/topic-substitution.yaml
```

You can point the `topic.substitution.file` to a file. e.g. file:/tmp/config/topic-substitution.yaml

# Supported cloud connectors

Kafka, ActiveMQ and Tibco support mapping of system topics to pre-allocated topics.

# Substitution file example

# ActiveMQ and Tibco

```yaml
#
# topic substitution example
#
# The default service monitor and multiplex topics are prefixed as service.monitor and multiplex.n
# where n starts from 0001
#
service:
  monitor:
    0: some.topic.one
    1: some.topic.two
    2: some.topic.three
    3: some.topic.four

#
# the 4-digit segment ID must be quoted to preserve the number of digits when the system parses this config file
#
multiplex:
  "0001":
    0: user.topic.one
    1: user.topic.two
    2: user.topic.three
    3: user.topic.four
    4: user.topic.five
    5: user.topic.six
```

# Kafka

Since Kafka is a native pub/sub system with partitioning support, you can specify the partition number for each replacement topic to map to the system topics (service.monitor.n and multiple.x.y).

If you do not provide the partition number, it is assumed to be the first partition. i.e. partition-0.

```yaml
#
# topic substitution example
#
# The default service monitor and multiplex topics are prefixed as service.monitor and multiplex.n
# where n starts from 0001
#
# PARTITION DEFINITION
# --------------------
# For the replacement topic, an optional partition number can be defined using the "#n" suffix.
# Partition number must be a positive number from 0 to the max partition of the specific topic.
#
# If partition number if not given, the entire topic will be used to send/receive events.
# e.g.
# SERVICE.APP.ONE is a topic of at least one partition
# user.app#0 is the topic "user.app" and partition "0"
#
service:
  monitor:
    0: SERVICE.APP.ONE
    1: SERVICE.APP.TWO
    2: SERVICE.APP.THREE
    3: SERVICE.APP.FOUR

#
# the 4-digit segment ID must be quoted to preserve the number of digits when the system parses this config file
#
multiplex:
  "0001":
    0: user.app#0
    1: user.app#1
    2: user.app#2
    3: user.app#3
    4: user.app#4
    5: user.app#5
```