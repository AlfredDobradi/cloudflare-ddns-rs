# cloudflare-ddns-rs

Dynamic DNS updater for CloudFlare hosted zones.

## Disclaimer
This is a learning project for me and should not be used in critical environments as there can and probably will be issues.

## Usage

Get an API Token with Zone.Zone:Read and Zone.DNS:edit scopes, fill config.json and let it rip.
The application uses the [httpbin.org](https://httpbin.org) API to determine public IP.

It won't update records that are:
1. Not A records
2. Not in the records list for the zone
3. Already have the determined IP

## Plans

* Make the code pretty
* Introduce non-blocking requests
* Become a real boy