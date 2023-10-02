use std::{net::SocketAddr, time::Duration};
use serde::Serialize;

use crate::error::ConfigurationError;

use self::raw_config::RawConfig;

pub mod auth;
pub mod raw_config;

pub enum ClientPropertyKey {
    BootstrapServersKey,
    ReconnectBackoffConfigKey,
    ReconnectBackoffMaxConfigKey,
}

#[derive(Debug, Serialize, Clone)]
pub enum ClientDnsLookup {
    UseAllDnsIps,
    ResolveCanonicalBootstrapServersOnly,
}

impl TryFrom<&str> for ClientDnsLookup {
    type Error = ConfigurationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "use_all_dns_ips" => Ok(ClientDnsLookup::UseAllDnsIps),
            "resolve_canonical_bootstrap_servers_only" => Ok(ClientDnsLookup::ResolveCanonicalBootstrapServersOnly),
            other => Err(ConfigurationError::InvalidClientDnsLookup(other.to_string())),
        }
    }
}

impl TryFrom<Result<&str, ConfigurationError>> for ClientDnsLookup {
    type Error = ConfigurationError;

    fn try_from(value: Result<&str, ConfigurationError>) -> Result<Self, Self::Error> {
        match value? {
            "use_all_dns_ips" => Ok(ClientDnsLookup::UseAllDnsIps),
            "resolve_canonical_bootstrap_servers_only" => Ok(ClientDnsLookup::ResolveCanonicalBootstrapServersOnly),
            other => Err(ConfigurationError::InvalidClientDnsLookup(other.to_string())),
        }
    }
}

// TODO: verify the correctness of this enum
#[derive(Debug, Serialize, Clone)]
pub enum MetricsRecordingLevel {
    Info,
    Debug,
    Trace,
    Error
}

pub enum SecurityProtocol {
    Plaintext,
    Mtls,
    Sasl(SaslMethod)
}

pub enum SaslMethod {
    Plain,
    ScramSha256,
    ScramSha512,
    Gssapi,
    OAuthBearer
}

type JavaClass = String;

#[derive(Debug, Serialize, Clone)]
pub enum ClientProperty {
    BootstrapServers(Vec<SocketAddr>),
    ClientDnsLookup(ClientDnsLookup),
    MetadataMaxAgeMs(Duration),
    SendBufferBytes(i32),
    RecvBufferBytes(i32),
    ClientId(String),
    ClientRack(String),
    Retries(i32),
    RetriesBackoffMs(Duration),
    ReconnectBackoffConfig(Duration),
    ReconnectBackoffMaxConfig(Duration),
    MetricsSampleWindowMs(Duration),
    MetricsNumSamples(i32),
    MetricsRecordingLevel(MetricsRecordingLevel),
    MetricReporterClasses(Vec<JavaClass>), // This will have to be modified 
    SocketConnectionSetupTimeoutMs(Duration),
    SocketConnectionSetupTimeoutMaxMs(Duration),
    ConnectionsMaxIdleMs(Duration),
    RequestTimeoutMs(Duration),
    DefaultListKeySerdeInner(JavaClass), // This will have to be modified as the type is javaclass
    DefaultListValueSerdeInner(JavaClass), // This will have to be modified as the type is javaclass
    DefaultListKeySerdeType(JavaClass), // This will have to be modified as the type is javaclass
    DefaultListValueSerdeType(JavaClass), // This will have to be modified as the type is javaclass
    GroupId(String),
    GroupInstanceId(String),
    MaxPollIntervalMs(Duration),
    RebalanceTimeoutMs(Duration),
    SessionTimeoutMs(Duration),
    HeartbeatIntervalMs(Duration),
    DefaultApiTimeoutMs(Duration),
    None
}

fn prep_key(value: &str) -> Result<&str, ConfigurationError> {
    match value.split("=").nth(0) {
        Some(val) => {
            let v = val.trim();

            if v.is_empty() {
                return Err(ConfigurationError::InvalidKey(String::from("Key is Empty")));
            }

            Ok(val.trim())
        },
        None => Err(ConfigurationError::MissingKey(value.to_string())),
    }
    
}

fn prep_value(value: &str) -> Result<&str, ConfigurationError> {
    match value.split("=").nth(1) {
        Some(val) =>  {
            let v = val.trim();

            if v.is_empty() {
                return Err(ConfigurationError::InvalidKey(String::from("Key is Empty")));
            }

            Ok(val.trim())
        },
        None => Err(ConfigurationError::MissingValue(value.to_string())),
    }
}

impl TryFrom<&str> for ClientProperty {
    type Error = ConfigurationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            raw if raw.starts_with("bootstrap.servers") => {
                let val = prep_value(raw)?;
                let mut servers = Vec::new();

                for server in val.split(",") {
                    let addr = match server.parse::<SocketAddr>() {
                        Ok(addr) => addr,
                        Err(e) => return Err(ConfigurationError::InvalidValueFor(raw.to_string(), e.to_string())),
                    };
                    servers.push(addr);
                }

                Ok(ClientProperty::BootstrapServers(servers))
            },
            raw if raw.starts_with("client.dns.lookup") => {
                let val: ClientDnsLookup = prep_value(raw).try_into()?;

                Ok(ClientProperty::ClientDnsLookup(val))
            },
            raw if raw.starts_with("metadata.max.age.ms") => {
                let val = match prep_value(raw)?.parse::<u64>() {
                    Ok(val) => val,
                    Err(e) => return Err(ConfigurationError::InvalidValueFor(raw.to_string(), e.to_string())),
                };

                Ok(ClientProperty::MetadataMaxAgeMs(Duration::from_millis(val)))
            },
            raw if raw.starts_with("send.buffer.bytes") => {
                let val = match prep_value(raw)?.parse::<i32>() {
                    Ok(val) => val,
                    Err(e) => return Err(ConfigurationError::InvalidValueFor(raw.to_string(), e.to_string())),
                };

                if val < SEND_BUFFER_LOWER_BOUND {
                    return Err(ConfigurationError::MalformedOption(raw.to_string()));
                }

                Ok(ClientProperty::SendBufferBytes(val))
            },
            raw if raw.starts_with("receive.buffer.bytes") => unimplemented!(""),
            raw if raw.starts_with("client.id") => unimplemented!(""),
            raw if raw.starts_with("client.rack") => unimplemented!(""),
            raw if raw.starts_with("reconnect.backoff.ms") => unimplemented!(""),
            raw if raw.starts_with("reconnect.backoff.max.ms") => unimplemented!(""),
            raw if raw.starts_with("retries") => unimplemented!(""),
            raw if raw.starts_with("retry.backoff.ms") => unimplemented!(""),
            raw if raw.starts_with("metrics.sample.window.ms") => unimplemented!(""),
            raw if raw.starts_with("metrics.num.samples") => unimplemented!(""),
            raw if raw.starts_with("metrics.recording.level") => unimplemented!(""),
            raw if raw.starts_with("metric.reporters") => unimplemented!(""),
            raw if raw.starts_with("auto.include.jmx.reporter") => unimplemented!(""),
            raw if raw.starts_with("security.protocol") => unimplemented!(""),
            raw if raw.starts_with("socket.connection.setup.timeout.ms") => unimplemented!(""),
            raw if raw.starts_with("socket.connection.setup.timeout.max.ms") => unimplemented!(""),
            raw if raw.starts_with("connections.max.idle.ms") => unimplemented!(""),
            raw if raw.starts_with("request.timeout.ms") => unimplemented!(""),
            raw if raw.starts_with("group.id") => unimplemented!(""),
            raw if raw.starts_with("group.instance.id") => unimplemented!(""),
            raw if raw.starts_with("max.poll.interval.ms") => unimplemented!(""),
            raw if raw.starts_with("rebalance.timeout.ms") => unimplemented!(""),
            raw if raw.starts_with("session.timeout.ms") => unimplemented!(""),
            raw if raw.starts_with("heartbeat.interval.ms") => unimplemented!(""),
            raw if raw.starts_with("default.api.timeout.ms") => unimplemented!(""),
            &_ => {
                // Move to the config parsing function.
                #[cfg(feature = "fail_invalid_configs")] 
                {

                }

                Err(ConfigurationError::UnrecognisedKey(value.to_string()))
            },
        }
    }
}

pub trait ClientConfig {
    fn get(&self, prop: ClientPropertyKey) -> &ClientProperty;
    fn push(&mut self, prop: ClientProperty) -> ClientProperty;
    fn pop(&mut self, prop: ClientPropertyKey) -> ClientProperty;
    fn contains(&self, prop: ClientPropertyKey) -> bool;
    fn new() -> Self;
}

const BOOTSTRAP_SERVERS_CONFIG: &str = "bootstrap.servers";
const BOOTSTRAP_SERVERS_DOC: &str = "A list of host/port pairs to use for establishing the initial connection to the Kafka cluster. The client will make use of all servers irrespective of which servers are specified here for bootstrapping&mdash;this list only impacts the initial hosts used to discover the full set of servers. This list should be in the form
    <code>host1:port1,host2:port2,...</code>. Since these servers are just used for the initial connection to
    discover the full cluster membership (which may change dynamically), this list need not contain the full set of
    servers (you may want more than one, though, in case a server is down).";

const CLIENT_DNS_LOOKUP_CONFIG: &str = "client.dns.lookup";
const CLIENT_DNS_LOOKUP_DOC: &str = "Controls how the client uses DNS lookups.
    If set to <code>use_all_dns_ips</code>, connect to each returned IP
    address in sequence until a successful connection is established.
    After a disconnection, the next IP is used. Once all IPs have been
    used once, the client resolves the IP(s) from the hostname again
    (both the JVM and the OS cache DNS name lookups, however).
    If set to <code>resolve_canonical_bootstrap_servers_only</code>,
    resolve each bootstrap address into a list of canonical names. After
    the bootstrap phase, this behaves the same as <code>use_all_dns_ips</code>.";

const METADATA_MAX_AGE_CONFIG: &str = "metadata.max.age.ms";
const METADATA_MAX_AGE_DOC: &str = "The period of time in milliseconds after which we force a refresh of metadata even if we haven't seen any partition leadership changes to proactively discover any new brokers or partitions.";

const SEND_BUFFER_CONFIG: &str = "send.buffer.bytes";
const SEND_BUFFER_DOC: &str = "The size of the TCP send buffer (SO_SNDBUF) to use when sending data. If the value is -1, the OS default will be used.";
const SEND_BUFFER_LOWER_BOUND: i32 = -1;

const RECEIVE_BUFFER_CONFIG: &str = "receive.buffer.bytes";
const RECEIVE_BUFFER_DOC: &str = "The size of the TCP receive buffer (SO_RCVBUF) to use when reading data. If the value is -1, the OS default will be used.";
const RECEIVE_BUFFER_LOWER_BOUND: i32 = -1;

const CLIENT_ID_CONFIG: &str = "client.id";
const CLIENT_ID_DOC: &str = "An id string to pass to the server when making requests. The purpose of this is to be able to track the source of requests beyond just ip/port by allowing a logical application name to be included in server-side request logging.";

const CLIENT_RACK_CONFIG: &str = "client.rack";
const CLIENT_RACK_DOC: &str = "A rack identifier for this client. This can be any string value which indicates where this client is physically located. It corresponds with the broker config 'broker.rack'";
const DEFAULT_CLIENT_RACK: &str = "";

const RECONNECT_BACKOFF_MS_CONFIG: &str = "reconnect.backoff.ms";
const RECONNECT_BACKOFF_MS_DOC: &str = "The base amount of time to wait before attempting to reconnect to a given host. This avoids repeatedly connecting to a host in a tight loop. This backoff applies to all connection attempts by the client to a broker.";

const RECONNECT_BACKOFF_MAX_MS_CONFIG: &str = "reconnect.backoff.max.ms";
const RECONNECT_BACKOFF_MAX_MS_DOC: &str = "The maximum amount of time in milliseconds to wait when reconnecting to a broker that has repeatedly failed to connect. If provided, the backoff per host will increase exponentially for each consecutive connection failure, up to this maximum. After calculating the backoff increase, 20% random jitter is added to avoid connection storms.";

const RETRIES_CONFIG: &str = "retries";
const RETRIES_DOC: &str = "Setting a value greater than zero will cause the client to resend any request that fails with a potentially transient error.
         It is recommended to set the value to either zero or `MAX_VALUE` and use corresponding timeout parameters to control how long a client should retry a request.";

const RETRY_BACKOFF_MS_CONFIG: &str = "retry.backoff.ms";
const RETRY_BACKOFF_MS_DOC: &str = "The amount of time to wait before attempting to retry a failed request to a given topic partition. This avoids repeatedly sending requests in a tight loop under some failure scenarios.";

const METRICS_SAMPLE_WINDOW_MS_CONFIG: &str = "metrics.sample.window.ms";
const METRICS_SAMPLE_WINDOW_MS_DOC: &str = "The window of time a metrics sample is computed over.";

const METRICS_NUM_SAMPLES_CONFIG: &str = "metrics.num.samples";
const METRICS_NUM_SAMPLES_DOC: &str = "The number of samples maintained to compute metrics.";

const METRICS_RECORDING_LEVEL_CONFIG: &str = "metrics.recording.level";
const METRICS_RECORDING_LEVEL_DOC: &str = "The highest recording level for metrics.";

const METRIC_REPORTER_CLASSES_CONFIG: &str = "metric.reporters";
const METRIC_REPORTER_CLASSES_DOC: &str = "A list of classes to use as metrics reporters. Implementing the <code>org.apache.kafka.common.metrics.MetricsReporter</code> interface allows plugging in classes that will be notified of new metric creation. The JmxReporter is always included to register JMX statistics.";

const METRICS_CONTEXT_PREFIX: &str = "metrics.context.";

#[deprecated]
const AUTO_INCLUDE_JMX_REPORTER_CONFIG: &str = "auto.include.jmx.reporter";
const AUTO_INCLUDE_JMX_REPORTER_DOC: &str = "Deprecated. Whether to automatically include JmxReporter even if it's not listed in <code>metric.reporters</code>. This configuration will be removed in Kafka 4.0, users should instead include <code>org.apache.kafka.common.metrics.JmxReporter</code> in <code>metric.reporters</code> in order to enable the JmxReporter.";

const SECURITY_PROTOCOL_CONFIG: &str = "security.protocol";
const SECURITY_PROTOCOL_DOC: &str = concat!("Protocol used to communicate with brokers. Valid values are: {}.", 
    "todo.security.protocols"); // SECURITY_PROTOCOL_CONFIG).as_str(); //TODO: Utils.join(SecurityProtocol.names(), ", ");
//const DEFAULT_SECURITY_PROTOCOL: &str = "PLAINTEXT";
const DEFAULT_SECURITY_PROTOCOL: SecurityProtocol = SecurityProtocol::Plaintext;

const SOCKET_CONNECTION_SETUP_TIMEOUT_MS_CONFIG: &str = "socket.connection.setup.timeout.ms";
const SOCKET_CONNECTION_SETUP_TIMEOUT_MS_DOC: &str = "The amount of time the client will wait for the socket connection to be established. If the connection is not built before the timeout elapses, clients will close the socket channel.";
const DEFAULT_SOCKET_CONNECTION_SETUP_TIMEOUT_MS: Duration = Duration::from_secs(10);

const SOCKET_CONNECTION_SETUP_TIMEOUT_MAX_MS_CONFIG: &str = "socket.connection.setup.timeout.max.ms";
const SOCKET_CONNECTION_SETUP_TIMEOUT_MAX_MS_DOC: &str = "The maximum amount of time the client will wait for the socket connection to be established. The connection setup timeout will increase exponentially for each consecutive connection failure up to this maximum. To avoid connection storms, a randomization factor of 0.2 will be applied to the timeout resulting in a random range between 20% below and 20% above the computed value.";
const DEFAULT_SOCKET_CONNECTION_SETUP_TIMEOUT_MAX_MS: Duration = Duration::from_secs(30);

const CONNECTIONS_MAX_IDLE_MS_CONFIG: &str = "connections.max.idle.ms";
const CONNECTIONS_MAX_IDLE_MS_DOC: &str = "Close idle connections after the number of milliseconds specified by this config.";

const REQUEST_TIMEOUT_MS_CONFIG: &str = "request.timeout.ms";
const REQUEST_TIMEOUT_MS_DOC: &str = "The configuration controls the maximum amount of time the client will wait
    for the response of a request. If the response is not received before the timeout
    elapses the client will resend the request if necessary or fail the request if
    retries are exhausted.";

const DEFAULT_LIST_KEY_SERDE_INNER_CLASS: &str = "default.list.key.serde.inner";
const DEFAULT_LIST_KEY_SERDE_INNER_CLASS_DOC: &str = "Default inner class of list serde for key that implements the <code>org.apache.kafka.common.serialization.Serde</code> interface. 
    This configuration will be read if and only if <code>default.key.serde</code> configuration is set to <code>org.apache.kafka.common.serialization.Serdes.ListSerde</code>";

const DEFAULT_LIST_VALUE_SERDE_INNER_CLASS: &str = "default.list.value.serde.inner";
const DEFAULT_LIST_VALUE_SERDE_INNER_CLASS_DOC: &str = "Default inner class of list serde for value that implements the <code>org.apache.kafka.common.serialization.Serde</code> interface.
    This configuration will be read if and only if <code>default.value.serde</code> configuration is set to <code>org.apache.kafka.common.serialization.Serdes.ListSerde</code>";

const DEFAULT_LIST_KEY_SERDE_TYPE_CLASS: &str = "default.list.key.serde.type";
const DEFAULT_LIST_KEY_SERDE_TYPE_CLASS_DOC: &str = concat!("Default class for key that implements the <code>java.util.List</code> interface. 
    This configuration will be read if and only if <code>default.key.serde</code> configuration is set to <code>org.apache.kafka.common.serialization.Serdes.ListSerde</code> 
    Note when list serde class is used, one needs to set the inner serde class that implements the <code>org.apache.kafka.common.serialization.Serde</code> interface via 
    '{}'", "default.list.key.serde.inner"); // DEFAULT_LIST_KEY_SERDE_INNER_CLASS

const DEFAULT_LIST_VALUE_SERDE_TYPE_CLASS: &str = "default.list.value.serde.type";
const DEFAULT_LIST_VALUE_SERDE_TYPE_CLASS_DOC: &str = concat!("Default class for value that implements the <code>java.util.List</code> interface. 
    This configuration will be read if and only if <code>default.value.serde</code> configuration is set to <code>org.apache.kafka.common.serialization.Serdes.ListSerde</code> 
    Note when list serde class is used, one needs to set the inner serde class that implements the <code>org.apache.kafka.common.serialization.Serde</code> interface via '
    '{}'", "default.list.value.serde.inner"); // DEFAULT_LIST_VALUE_SERDE_INNER_CLASS

const GROUP_ID_CONFIG: &str = "group.id";
const GROUP_ID_DOC: &str = "A unique string that identifies the consumer group this consumer belongs to. This property is required if the consumer uses either the group management functionality by using <code>subscribe(topic)</code> or the Kafka-based offset management strategy.";

const GROUP_INSTANCE_ID_CONFIG: &str = "group.instance.id";
const GROUP_INSTANCE_ID_DOC: &str = "A unique identifier of the consumer instance provided by the end user. 
    Only non-empty strings are permitted. If set, the consumer is treated as a static member, 
    which means that only one instance with this ID is allowed in the consumer group at any time. 
    This can be used in combination with a larger session timeout to avoid group rebalances caused by transient unavailability 
    (e.g. process restarts). If not set, the consumer will join the group as a dynamic member, which is the traditional behavior.";

const MAX_POLL_INTERVAL_MS_CONFIG: &str = "max.poll.interval.ms";
const MAX_POLL_INTERVAL_MS_DOC: &str = "The maximum delay between invocations of poll() when using 
    consumer group management. This places an upper bound on the amount of time that the consumer can be idle 
    before fetching more records. If poll() is not called before expiration of this timeout, then the consumer 
    is considered failed and the group will rebalance in order to reassign the partitions to another member. 
    For consumers using a non-null <code>group.instance.id</code> which reach this timeout, partitions will not be immediately reassigned. 
    Instead, the consumer will stop sending heartbeats and partitions will be reassigned 
    after expiration of <code>session.timeout.ms</code>. This mirrors the behavior of a static consumer which has shutdown.";

const REBALANCE_TIMEOUT_MS_CONFIG: &str = "rebalance.timeout.ms";
const REBALANCE_TIMEOUT_MS_DOC: &str = "The maximum allowed time for each worker to join the group 
    once a rebalance has begun. This is basically a limit on the amount of time needed for all tasks to 
    flush any pending data and commit offsets. If the timeout is exceeded, then the worker will be removed 
    from the group, which will cause offset commit failures.";

const SESSION_TIMEOUT_MS_CONFIG: &str = "session.timeout.ms";
const SESSION_TIMEOUT_MS_DOC: &str = "The timeout used to detect client failures when using 
    Kafka's group management facility. The client sends periodic heartbeats to indicate its liveness 
    to the broker. If no heartbeats are received by the broker before the expiration of this session timeout, 
    then the broker will remove this client from the group and initiate a rebalance. Note that the value 
    must be in the allowable range as configured in the broker configuration by <code>group.min.session.timeout.ms</code> 
    and <code>group.max.session.timeout.ms</code>.";

const HEARTBEAT_INTERVAL_MS_CONFIG: &str = "heartbeat.interval.ms";
const HEARTBEAT_INTERVAL_MS_DOC: &str = "The expected time between heartbeats to the consumer 
    coordinator when using Kafka's group management facilities. Heartbeats are used to ensure that the 
    consumer's session stays active and to facilitate rebalancing when new consumers join or leave the group. 
    The value must be set lower than <code>session.timeout.ms</code>, but typically should be set no higher 
    than 1/3 of that value. It can be adjusted even lower to control the expected time for normal rebalances.";

const DEFAULT_API_TIMEOUT_MS_CONFIG: &str = "default.api.timeout.ms";
const DEFAULT_API_TIMEOUT_MS_DOC: &str = "Specifies the timeout (in milliseconds) for client APIs.
    This configuration is used as the default timeout for all client operations that do not specify a <code>timeout</code> parameter.";


/**
 * Postprocess the configuration so that exponential backoff is disabled when reconnect backoff
 * is explicitly configured but the maximum reconnect backoff is not explicitly configured.
 */
pub fn post_process_reconnect_backoff_config_inplace<T>(config: &mut T) 
where T: ClientConfig
{
    use ClientPropertyKey::*;
    use ClientProperty::*;

    if config.contains(ReconnectBackoffConfigKey)
    && !config.contains(ReconnectBackoffMaxConfigKey)
    {
        if let ReconnectBackoffConfig(base) = config.get(
            ReconnectBackoffConfigKey
        ) {
            config.push(
                ReconnectBackoffMaxConfig(*base)
            );
        }
    };
}

pub fn post_process_reconnect_backoff_configs<T>(config: & T) -> T
where T: ClientConfig + Clone
{
    use ClientPropertyKey::*;
    use ClientProperty::*;

    let mut new_config: T = config.clone();

    if config.contains(ReconnectBackoffConfigKey)
    && !config.contains(ReconnectBackoffMaxConfigKey)
    {
        if let ReconnectBackoffConfig(base) = config.get(
            ReconnectBackoffConfigKey
        ) {
            new_config.push(
                ReconnectBackoffMaxConfig(*base)
            );
        }
    };

    new_config
}

    /*public static Map<String, Object> post_process_reconnect_backoff_configs(AbstractConfig config,
                                                    Map<String, Object> parsedValues) {
        HashMap<String, Object> rval = new HashMap<>();
        Map<String, Object> originalConfig = config.originals();
        if ((!originalConfig.containsKey(RECONNECT_BACKOFF_MAX_MS_CONFIG)) &&
            originalConfig.containsKey(RECONNECT_BACKOFF_MS_CONFIG)) {
            log.debug("Disabling exponential reconnect backoff because {} is set, but {} is not.",
                    RECONNECT_BACKOFF_MS_CONFIG, RECONNECT_BACKOFF_MAX_MS_CONFIG);
            rval.put(RECONNECT_BACKOFF_MAX_MS_CONFIG, parsedValues.get(RECONNECT_BACKOFF_MS_CONFIG));
        }
        return rval;
    }

    public static void postValidateSaslMechanismConfig(AbstractConfig config) {
        SecurityProtocol securityProtocol = SecurityProtocol.forName(config.getString(CommonClientConfigs.SECURITY_PROTOCOL_CONFIG));
        String clientSaslMechanism = config.getString(SaslConfigs.SASL_MECHANISM);
        if (securityProtocol == SecurityProtocol.SASL_PLAINTEXT || securityProtocol == SecurityProtocol.SASL_SSL) {
            if (clientSaslMechanism == null || clientSaslMechanism.isEmpty()) {
                throw new ConfigException(SaslConfigs.SASL_MECHANISM, null, "When the " + CommonClientConfigs.SECURITY_PROTOCOL_CONFIG +
                        " configuration enables SASL, mechanism must be non-null and non-empty string.");
            }
        }
    }

    public static List<MetricsReporter> metricsReporters(AbstractConfig config) {
        return metricsReporters(Collections.emptyMap(), config);
    }

    public static List<MetricsReporter> metricsReporters(String clientId, AbstractConfig config) {
        return metricsReporters(Collections.singletonMap(CommonClientConfigs.CLIENT_ID_CONFIG, clientId), config);
    }

    public static List<MetricsReporter> metricsReporters(Map<String, Object> clientIdOverride, AbstractConfig config) {
        List<MetricsReporter> reporters = config.getConfiguredInstances(CommonClientConfigs.METRIC_REPORTER_CLASSES_CONFIG,
                MetricsReporter.class, clientIdOverride);
        if (config.getBoolean(CommonClientConfigs.AUTO_INCLUDE_JMX_REPORTER_CONFIG) &&
                reporters.stream().noneMatch(r -> JmxReporter.class.equals(r.getClass()))) {
            JmxReporter jmxReporter = new JmxReporter();
            jmxReporter.configure(config.originals(clientIdOverride));
            reporters.add(jmxReporter);
        }
        return reporters;
    }*/

mod config_tests {
    use std::net::SocketAddrV4;

    use super::*;

    #[test]
    fn test_prep_key() {
        let key = "bootstrap.servers=";
        let key = prep_key(key).unwrap();
        assert_eq!(key, "bootstrap.servers");
    }

    #[test]
    fn test_prep_value() {
        let value = "bootstrap.servers=192.168.0.1:9092";
        let value = prep_value(value).unwrap();
        assert_eq!(value, "192.168.0.1:9092");
    }

    #[test]
    fn test_failed_prep_value() {
        let mut value = prep_value("bootstrap.servers");
        assert!(value.is_err());
        value = prep_value("bootstrap.servers=");
        assert!(value.is_err());
    }

    #[test]
    fn test_failed_prep_key() {
        let mut value = prep_key("");
        assert!(value.is_err());        
        value = prep_key("=");
        assert!(value.is_err());
        value = prep_key("=askdnsd");
        assert!(value.is_err());
    }

    #[test]
    fn test_client_propert_try_from() {
        let mut raw = "bootstrap.servers=192.168.0.1:9092";

        let client_prop_bs_serv = dbg!(ClientProperty::try_from(raw));

        assert!(client_prop_bs_serv.is_ok());

        match ClientProperty::try_from(raw) {
            Err(_) => unimplemented!(),
            Ok(ClientProperty::BootstrapServers(val)) => {
                let reference = vec![
                    SocketAddr::new([192, 168, 0, 1].into(), 9092)
                ];

                assert!(val.eq(&reference));
            },
            Ok(_) => assert!(false)
        }

        raw = "bootstrap.servers=192.168.0.1:9092,192.168.0.2:9093";

        let client_prop_bs_serv_multi = dbg!(ClientProperty::try_from(raw));

        assert!(client_prop_bs_serv_multi.is_ok());

        match ClientProperty::try_from(raw) {
            Err(_) => unimplemented!(),
            Ok(ClientProperty::BootstrapServers(val)) => {
                let reference = vec![
                    SocketAddr::new([192, 168, 0, 1].into(), 9092),
                    SocketAddr::new([192, 168, 0, 2].into(), 9093)
                ];

                assert!(val.eq(&reference));
            },
            Ok(_) => assert!(false)
        }

        raw = "bootstrap.servers=192.168.0.1";
        let client_prop_bs_serv_fail = dbg!(ClientProperty::try_from(raw));
        assert!(client_prop_bs_serv_fail.is_err());

        raw = "bootstrap.server=192.168.0.1";
        let client_prop_key_fail = dbg!(ClientProperty::try_from(raw));
        assert!(client_prop_bs_serv_fail.is_err());
    }

    #[test]
    fn test_ser() {
        let bs = ClientProperty::BootstrapServers(vec![
            SocketAddr::new([192, 168, 0, 1].into(), 9092),
            SocketAddr::new([192, 168, 0, 2].into(), 9093)
        ]);
        let bs_json = serde_json::to_string(&bs).unwrap();
        dbg!(bs_json);

        let dnsl = ClientProperty::ClientDnsLookup(ClientDnsLookup::UseAllDnsIps);
        let dnsl_json = serde_json::to_string(&dnsl).unwrap();
        dbg!(dnsl_json);

        let v = vec![bs, dnsl];
        let v_json = serde_json::to_string(&v).unwrap();
        dbg!(v_json);
    }
}