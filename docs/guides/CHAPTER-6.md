# Cloud connectors

Mercury has been integrated and tested with both event stream systems and enterprise service bus messaging systems.

1. Event stream system - Apache Kafka
2. Messaging system - Hazelcast
3. Enterprise Service Bus - ActiveMQ artemis and Tibco EMS

# Reusable topics

The default topics include the following:

1. service.monitor.n
2. multiplex.x.y

where `service.monitor.0` is used by the presence monitor to communicate with its peers for fault tolerant operation.
service.monitor.1 and higher topics are used for closed user groups. i.e. service.monitor.1 for closed user group 1 and
service.monitor.2 for closed user group 2, etc.

Usually all user application instances should use the same closed user group unless you want to logically segregate 
different application domains into their own closed user groups. Application modules in one group are invisible to
another group.

multiplex.x.y topics are used by user application instances. The presence monitor will assign one topic to one 
application instance dynamically and release the topic when the application instance leaves the system. 
This allows topics to be reused automatically.

There is a RSVP reservation protocol that the presence monitors will coordinate with each other to assign a 
unique topic to each application instance.

The value `x` must be a 4-digit number starting from 0001 and `y` is the partition number for the topic. 
If the underlying messaging system supports pub/sub, the partition number maps to the physical partition of a topic. 
For enterprise service bus, the partition number is a logical identifier that corresponds to a physical topic.

# Topic Substitution

Some enterprises do not allow automatic creation of messaging topics by user applications.

In this case, you can turn on topic substitution with the following parameters in application.properties

```
application.feature.topic.substitution=true
topic.substitution.file=file:/tmp/config/topic-substitution.yaml,classpath:/topic-substitution.yaml
```

You can point the `topic.substitution.file` to a file.

## Supported cloud connectors

Kafka, ActiveMQ and Tibco support mapping of system topics to pre-allocated topics. 
Hazelcast topics are generated dynamically and thus topic pre-allocation is not required.

Let's examine the configuration concept with two sample topic substitution files.

### ActiveMQ and Tibco

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

### Kafka

Since Kafka is a pub/sub event streaming system with partitioning support, you can use the "#n" suffix to specify 
the partition number for each replacement topic to map to the system topics (service.monitor.n and multiple.x.y). 
(Note that the "#n" syntax is not applicable to ActiveMQ and Tibco connectors above)

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
    0: SERVICE.APP.ONE#0
    1: SERVICE.APP.TWO#0
    2: SERVICE.APP.THREE#0
    3: SERVICE.APP.FOUR#0

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

---

| Chapter-7                           | Home                                     |
| :----------------------------------:|:----------------------------------------:|
| [Version Control](CHAPTER-7.md)     | [Table of Contents](TABLE-OF-CONTENTS.md)|
