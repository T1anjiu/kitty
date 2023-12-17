use anyhow::Result;
use sysproxy::Sysproxy;

#[cfg(target_os = "windows")]
pub fn set_system_proxy(host: &str, socks_port: u16, http_port: Option<u16>) -> Result<()> {
    use anyhow::anyhow;

    let res = if let Some(port) = http_port {
        let mut socks_sysproxy = Sysproxy {
            enable: true,
            host: host.into(),
            port: port,
            bypass: "localhost;127.*".into(),
        };

        let _ = socks_sysproxy.set_system_proxy();
        Ok(())
    } else {
        Err(anyhow!("windows must set http proxy port"))
    };
    res
}

#[cfg(target_os = "linux")]
pub fn set_system_proxy(host: &str, socks_port: u16, http_port: Option<u16>) -> Result<()> {
    let socks_sysproxy = Sysproxy {
        enable: true,
        host: host.into(),
        port: socks_port,
        bypass: "localhost,127.0.0.1/8".into(),
    };
    let _ = socks_sysproxy.set_enable()?;
    let _ = socks_sysproxy.set_socks();
    match http_port {
        Some(http_port) => {
            let socks_sysproxy = Sysproxy {
                enable: true,
                host: host.into(),
                port: http_port,
                bypass: "localhost,127.0.0.1/8".into(),
            };
            let _ = socks_sysproxy.set_http();
            let _ = socks_sysproxy.set_https();
        }
        None => (),
    }
    Ok(())
}

#[cfg(target_os = "macos")]
pub fn set_system_proxy(host: &str, socks_port: u16, http_port: Option<u16>) -> Result<&str> {
    use anyhow::Ok;

    let service = "Wi-Fi";
    let mut socks_sysproxy = Sysproxy {
        enable: true,
        host: host.into(),
        port: socks_port,
        bypass: "localhost,127.0.0.1/8".into(),
    };
    let _ = socks_sysproxy.set_socks("0x10");
    match http_port {
        Some(http_port) => {
            let mut socks_sysproxy = Sysproxy {
                enable: true,
                host: host.into(),
                port: http_port,
                bypass: "localhost,127.0.0.1/8".into(),
            };
            let _ = socks_sysproxy.set_http(service);
            let _ = socks_sysproxy.set_https(service);
            Ok("set socks proxy success")
        }
        None => Ok("the http_port is not set"),
    }
}

#[cfg(target_os = "windows")]
pub fn clear_system_proxy() {
    let mut socks_sysproxy = Sysproxy {
        enable: false,
        host: "127.0.0.1".into(),
        port: 10086,
        bypass: "localhost;127.*".into(),
        #[cfg(not(target_os = "windows"))]
        bypass: "localhost,127.0.0.1/8".into(),
    };
    let _ = socks_sysproxy.set_system_proxy();
}

#[cfg(target_os = "linux")]
pub fn clear_system_proxy() -> Result<()> {
    use std::process::Command;
    const CMD_KEY: &str = "org.gnome.system.proxy";

    Command::new("gsettings")
        .args(["set", CMD_KEY, "mode", "none"])
        .status()?;
    Ok(())
}

#[warn(dead_code)]
enum ProxyType {
    HTTP,
    HTTPS,
    SOCKS,
}

impl ProxyType {
    fn to_target(&self) -> &'static str {
        match self {
            ProxyType::HTTP => "webproxy",
            ProxyType::HTTPS => "securewebproxy",
            ProxyType::SOCKS => "socksfirewallproxy",
        }
    }
}

#[cfg(target_os = "macos")]
pub fn clear_system_proxy() -> Result<()> {
    use std::process::Command;
    let service = "Wi-Fi";
    let target_state = format!("-set{}state", ProxyType::HTTP.to_target());
    Command::new("networksetup")
        .args([target_state.as_str(), service, "off"])
        .status()?;
    Ok(())
}
