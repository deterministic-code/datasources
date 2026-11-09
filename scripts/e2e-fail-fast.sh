# Shared by e2e-docker.sh. Source it; do not execute.

e2e_fail_line() {
  case "$1" in
    *"0 Warning(s)"* | *"0 Error(s)"* | *"npm warn Unknown env config"* | *"npm warn deprecated"* | *"npm warn skipping integrity check"* | *"warning NU1902"* | *"warning: the following packages contain code that will be rejected by a future version of Rust"*) return 1 ;;
    *"warning:"* | *"error:"* | *"error["* | *": error "* | *"warning NU"* | *"npm warn"* | *"npm error"*) return 0 ;;
    *) return 1 ;;
  esac
}

e2e_watch() {
  while IFS= read -r line || [ -n "$line" ]; do
    printf '%s\n' "$line"
    if e2e_fail_line "$line"; then
      exec <&-
      return 1
    fi
  done
  return 0
}
