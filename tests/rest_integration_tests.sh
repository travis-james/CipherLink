#!/bin/bash
set -eu

echo "▶️ Starting /health test"
response=$(curl -sS http://localhost:3000/health)
status=$(echo "$response" | jq -r .status)
if [[ "$status" == "healthy" ]]; then
  echo "✅ /health passed"
else
  echo "❌ Unexpected status: $status"
  exit 1
fi


echo "▶️ Starting /encrypt test"

plain_text="http://yahoo.com"
key="foobar"
response=$(curl -s -X POST http://localhost:3000/encrypt \
  -H "Content-Type: application/json" \
  -d "{\"plain_text\":\"$plain_text\", \"key\":\"$key\"}")
status=$(echo "$response" | jq -r .status)
id=$(echo "$response" | jq -r .data.id)

if [[ "$status" != "Ok" ]]; then
  echo "❌ Unexpected status: $status"
  exit 1
fi
if [[ -z "$id" ]]; then
  echo "❌ No ID returned"
  exit 1
fi
echo "✅ Encrypt endpoint returned ID: $id"


echo "▶️ Starting /decrypt test"

# follow redirect, discard body, output final url after all redirects.
final_url=$(curl -s -L -o /dev/null -w "%{url_effective}" \
  "http://localhost:3000/decrypt/$id/$key")

# Strip protocol, www and trailing slashes from both
expected_domain=$(echo "$plain_text" | sed -E 's|https?://||; s|^www\.||; s|/$||')
actual_domain=$(echo "$final_url" | sed -E 's|https?://||; s|^www\.||; s|/$||')

if [[ "$actual_domain" == "$expected_domain"* ]]; then
  echo "✅ Redirected to expected domain: $final_url"
else
  echo "❌ Mismatch: expected domain $expected_domain, got $actual_domain"
  exit 1
fi