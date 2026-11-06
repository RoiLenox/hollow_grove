# Shared by the installed launchers. Keep fd 9 open until their runtime exits.
hg_device_lock() {
    local root="${XDG_DATA_HOME:-$HOME/.local/share}/hollow-grove"
    mkdir -p -- "$root"
    exec 9>"$root/.device.lock"
    if ! flock --shared --nonblock 9; then
        printf 'Hollow Grove state is being transferred; try again after handoff finishes.\n' >&2
        return 1
    fi
    if [[ -f "$root/.device-transaction.json" || -f "$root/.handoff-paused.json" ]]; then
        local message='This device is parked after a handoff, or needs recovery. Run hollow-grove-device status for the next step.'
        printf '%s\n' "$message" >&2
        if command -v notify-send >/dev/null 2>&1; then
            notify-send 'Hollow Grove handoff' "$message" || true
        fi
        return 1
    fi
}
