#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

#[cfg(feature = "cidr")]
pub use use_cidr as cidr;
#[cfg(feature = "dns")]
pub use use_dns as dns;
#[cfg(feature = "domain")]
pub use use_domain as domain;
#[cfg(feature = "host")]
pub use use_host as host;
#[cfg(feature = "ip")]
pub use use_ip as ip;
#[cfg(feature = "mac")]
pub use use_mac as mac;
#[cfg(feature = "port")]
pub use use_port as port;
#[cfg(feature = "socket")]
pub use use_socket as socket;
#[cfg(feature = "tcp")]
pub use use_tcp as tcp;
#[cfg(feature = "udp")]
pub use use_udp as udp;
