#!/usr/bin/env bash
set -euo pipefail

deploy_user="${1:-${SUDO_USER:-}}"
if [ -z "${deploy_user}" ] || [ "${deploy_user}" = "root" ]; then
  echo "Usage: sudo bash scripts/vps/InstallStaticWebRouter.sh <deploy-user>" >&2
  exit 1
fi

if ! id "${deploy_user}" >/dev/null 2>&1; then
  echo "Deploy user does not exist: ${deploy_user}" >&2
  exit 1
fi

export DEBIAN_FRONTEND=noninteractive
apt-get update
apt-get install -y nginx

mkdir -p /srv/apps /srv/www
chown "${deploy_user}:${deploy_user}" /srv/apps /srv/www
chmod 0755 /srv /srv/apps /srv/www

cat >/etc/nginx/sites-available/static-apps <<'NGINX'
server {
    listen 80 default_server;
    listen [::]:80 default_server;
    server_name _;

    root /srv/www;
    index index.html;

    location / {
        try_files $uri $uri/ =404;
    }
}
NGINX

timestamp="$(date +%Y%m%d%H%M%S)"
for enabled_site in /etc/nginx/sites-enabled/*; do
  if [ ! -e "${enabled_site}" ]; then
    continue
  fi
  if [ "$(basename "${enabled_site}")" = "static-apps" ]; then
    continue
  fi
  if grep -qs "default_server" "${enabled_site}"; then
    mv "${enabled_site}" "${enabled_site}.disabled-by-static-apps-${timestamp}"
  fi
done

ln -sfn /etc/nginx/sites-available/static-apps /etc/nginx/sites-enabled/static-apps
nginx -t
systemctl enable --now nginx
systemctl reload nginx

if command -v ufw >/dev/null 2>&1 && ufw status | grep -q "Status: active"; then
  ufw allow 80/tcp
fi

cat <<EOF
Static app router installed.

Public HTTP is now served from:
  /srv/www

Deployments can publish apps as:
  /srv/www/<app-name> -> /srv/apps/<app-name>/current

Browser URLs use:
  http://<vps-host>/<app-name>/
EOF
