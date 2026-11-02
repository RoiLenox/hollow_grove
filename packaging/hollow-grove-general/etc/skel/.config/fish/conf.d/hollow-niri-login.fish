# Start from the real local login session so niri-session can import its identity.
# Niri must not also be enabled under default.target.wants.
if status is-login; and status is-interactive; and not set -q SSH_CONNECTION; and not set -q HOLLOW_NIRI_SESSION_LAUNCH
    if test (tty) = /dev/tty1; and not systemctl --user --quiet is-active niri.service
        set -gx HOLLOW_NIRI_SESSION_LAUNCH 1
        exec /usr/bin/niri-session
    end
end
