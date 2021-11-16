MAVEN_CLI_OPTS="--show-version"
MAVEN_OPTS="-Dmaven.repo.local=./.m2/repository -Djava.awt.headless=true"

# System
mvn $MAVEN_CLI_OPTS clean compile install -f system/platform-core
mvn $MAVEN_CLI_OPTS clean compile install -f system/rest-spring

# Core
mvn $MAVEN_CLI_OPTS clean compile install -f connectors/core/cloud-connector
mvn $MAVEN_CLI_OPTS clean compile install -f connectors/core/service-monitor

# Kafka
mvn $MAVEN_CLI_OPTS clean compile install -f connectors/adapters/kafka/kafka-standalone
mvn $MAVEN_CLI_OPTS clean compile install -f connectors/adapters/kafka/kafka-presence
mvn $MAVEN_CLI_OPTS clean compile install -f connectors/adapters/kafka/kafka-connector

# Active MQ
mvn $MAVEN_CLI_OPTS clean compile install -f connectors/adapters/activemq/activemq-connector
mvn $MAVEN_CLI_OPTS clean compile install -f connectors/adapters/activemq/activemq-presence

# Hazelcast
mvn $MAVEN_CLI_OPTS clean compile install -f connectors/adapters/hazelcast/hazelcast-connector
mvn $MAVEN_CLI_OPTS clean compile install -f connectors/adapters/hazelcast/hazelcast-presence

# Tibco (requires libraries)
# mvn $MAVEN_CLI_OPTS clean compile install -f connectors/adapters/tibco/tibco-connector
# mvn $MAVEN_CLI_OPTS clean compile install -f connectors/adapters/tibco/tibco-presence

# Language Connector
mvn $MAVEN_CLI_OPTS clean compile install -f language-packs/language-connector

# Extensions
mvn $MAVEN_CLI_OPTS clean compile install -f extensions/api-playground
mvn $MAVEN_CLI_OPTS clean compile install -f extensions/distributed-tracer

mvn $MAVEN_CLI_OPTS clean compile install -f extensions/rest-automation-lib
mvn $MAVEN_CLI_OPTS clean compile install -f extensions/rest-automation-app

mvn $MAVEN_CLI_OPTS clean compile install -f extensions/simple-scheduler