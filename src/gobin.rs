/// gen script generate the script we need to install
pub fn gen_script(path: &str) -> String {
  let (pkg, _mod, ver, bin) = parse_package(path);
  // TODO: Resolve version
  return format!(
    crate::install_sh!(),
    api = GOBIN_API,
    pkg = pkg,
    ver = ver,
    over = ver,
    bin = bin,
  );
}

/// parse pakage reuturn package information
/// returns: pkg, mod, ver, bin
fn parse_package(path: &str) -> (String, String, String, String) {
  let parts: Vec<&str> = path[1..].split("@").map(|s| s).collect();

  // first one maybe the package, second one maybe the version
  // normalize package

  let mut pkg = parts[0].to_string();

  // normalize pkg with github.com or go.zoe.im
  // gopkg => go.zoe.im/gopkg
  // zeus/cmd/zeus => go.zoe.im/zeus/cmd/zeus, contains go.zoe.im/index.html
  // frp/frp => github.com/frp/frp
  let sparts: Vec<&str> = pkg.split("/").collect();
  match sparts.len() {
    0 => {}
    1 => pkg = format!("go.zoe.im/{}", pkg),
    2 => pkg = format!("github.com/{}", pkg),
    _ => {
      // if the third part is `cmd`, guess is my repo
      if sparts[1] == "cmd" {
        pkg = format!("go.zoe.im/{}", pkg);
      } else {
        pkg = format!("github.com/{}", pkg);
      }
    }
  }

  let mmod = "";

  // binary at the end of package
  let bin: String = if let Some(idx) = pkg.rfind("/") {
    pkg[idx + 1..].to_string()
  } else {
    pkg[..].to_string()
  };

  // version after @
  let mut ver: &str = "master";

  return (pkg, mmod.to_string(), ver.to_string(), bin);
}

static GOBIN_API: &'static str = "https://gobinaries.com";

#[macro_export(local_inner_macros)]
macro_rules! install_sh {
  () => {
    r#"
#!/bin/sh

set -e

echoerr() {{
  echo "$@" 1>&2
}}

log_info() {{
  echo "\033[38;5;61m  ==>\033[0;00m $@"
}}

log_crit() {{
  echoerr
  echoerr "  \033[38;5;125m$@\033[0;00m"
  echoerr
}}

is_command() {{
  command -v "$1" >/dev/null
  #type "$1" > /dev/null 2> /dev/null
}}

download_wget() {{
  local_file=$1
  source_url=$2
  header=$3
  if [ -z "$header" ]; then
    wget -q -O "$local_file" "$source_url"
  else
    wget -q --header "$header" -O "$local_file" "$source_url"
  fi
}}

download_curl() {{
  local_file=$1
  source_url=$2
  header=$3
  if [ -z "$header" ]; then
    code=$(curl -w '%{{http_code}}' -sL -o "$local_file" "$source_url")
  else
    code=$(curl -w '%{{http_code}}' -sL -H "$header" -o "$local_file" "$source_url")
  fi
  if [ "$code" != "200" ]; then
    log_crit "Error downloading, got $code response from server"
    return 1
  fi
  return 0
}}

download() {{
  if is_command curl; then
    download_curl "$@"
    return
  elif is_command wget; then
    download_wget "$@"
    return
  fi
  log_crit "http_download unable to find wget or curl"
  return 1
}}

http_copy() {{
  tmp=$(mktemp)
  download "${{tmp}}" "$1" "$2" || return 1
  body=$(cat "$tmp")
  rm -f "${{tmp}}"
  echo "$body"
}}

uname_os() {{
  os=$(uname -s | tr '[:upper:]' '[:lower:]')

  # fixed up for https://github.com/client9/shlib/issues/3
  case "$os" in
    msys_nt*) os="windows" ;;
    mingw*) os="windows" ;;
  esac

  # other fixups here
  echo "$os"
}}

uname_os_check() {{
  os=$(uname_os)
  case "$os" in
    darwin) return 0 ;;
    dragonfly) return 0 ;;
    freebsd) return 0 ;;
    linux) return 0 ;;
    android) return 0 ;;
    nacl) return 0 ;;
    netbsd) return 0 ;;
    openbsd) return 0 ;;
    plan9) return 0 ;;
    solaris) return 0 ;;
    windows) return 0 ;;
  esac
  log_crit "uname_os_check '$(uname -s)' got converted to '$os' which is not a GOOS value. Please file bug at https://github.com/client9/shlib"
  return 1
}}

uname_arch() {{
  arch=$(uname -m)
  case $arch in
    x86_64) arch="amd64" ;;
    x86) arch="386" ;;
    i686) arch="386" ;;
    i386) arch="386" ;;
    aarch64) arch="arm64" ;;
    armv5*) arch="armv5" ;;
    armv6*) arch="armv6" ;;
    armv7*) arch="armv7" ;;
  esac
  echo ${{arch}}
}}

uname_arch_check() {{
  arch=$(uname_arch)
  case "$arch" in
    386) return 0 ;;
    amd64) return 0 ;;
    arm64) return 0 ;;
    armv5) return 0 ;;
    armv6) return 0 ;;
    armv7) return 0 ;;
    ppc64) return 0 ;;
    ppc64le) return 0 ;;
    mips) return 0 ;;
    mipsle) return 0 ;;
    mips64) return 0 ;;
    mips64le) return 0 ;;
    s390x) return 0 ;;
    amd64p32) return 0 ;;
  esac
  log_crit "uname_arch_check '$(uname -m)' got converted to '$arch' which is not a GOARCH value.  Please file bug report at https://github.com/client9/shlib"
  return 1
}}

mktmpdir() {{
  test -z "$TMPDIR" && TMPDIR="$(mktemp -d)"
  mkdir -p "${{TMPDIR}}"
  echo "${{TMPDIR}}"
}}

start() {{
  uname_os_check
  uname_arch_check

  api="{api}"

  pkg="{pkg}"

  bin="{bin}"

  original_version="{over}"

  version="{ver}"
  
  prefix=${{PREFIX:-"/usr/local/bin"}}
  tmp="$(mktmpdir)/$bin"

  echo
  log_info "Downloading $pkg@$original_version"
  if [ "$original_version" != "$version" ]; then
    log_info "Resolved version $original_version to $version"
  fi
  log_info "Downloading binary for $os $arch"
  download $tmp "$api/binary/$pkg?os=$os&arch=$arch&version=$version"

  log_info "Installing $bin to $prefix"
  sudo install "$tmp" "$prefix"

  log_info "Installation complete"
  echo
}}

start
"#;
  };
}

// static ERROR_SH: &'static str = r#"
// echo
// echo "  \033[38;5;125mError:\033[0;00m {{.}}"
// echo
// exit 1
// "#;

#[cfg(test)]
mod tests {
  use super::parse_package;

  #[test]
  fn can_parse_ok() {
    assert_eq!(
      parse_package("/gopkg"),
      (
        "go.zoe.im/gopkg".to_string(),
        "".to_string(),
        "master".to_string(),
        "gopkg".to_string()
      )
    );
    assert_eq!(
      parse_package("/zeus/cmd/zeus"),
      (
        "go.zoe.im/zeus/cmd/zeus".to_string(),
        "".to_string(),
        "master".to_string(),
        "zeus".to_string()
      )
    );
    assert_eq!(
      parse_package("/frp/frp"),
      (
        "github.com/frp/frp".to_string(),
        "".to_string(),
        "master".to_string(),
        "frp".to_string()
      )
    );
    assert_eq!(
      parse_package("/frp/frp/cmd/frp"),
      (
        "github.com/frp/frp/cmd/frp".to_string(),
        "".to_string(),
        "master".to_string(),
        "frp".to_string()
      )
    );
  }
}
