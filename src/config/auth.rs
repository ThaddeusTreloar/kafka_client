

pub enum KafkaCredentials {
    SaslPlain(SaslPlainAuthentication),
    SaslsGssApi(SaslsGssApiAuthentication),
    Mtls(MtlsAuthentication),
    Plaintext,
}

pub struct SaslPlainAuthentication {}
pub struct SaslsGssApiAuthentication {}
pub struct MtlsAuthentication {}