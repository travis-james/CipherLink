#!/bin/bash
set -euo pipefail

plain_text="http://yahoo.com"
key="music"
expected="yahoo.com"


echo "▶️ Lambda /health test"
cat > lambda_event.json <<EOF
{
  "httpMethod": "GET",
  "path": "/health"
}
EOF
response=$(cargo lambda invoke --data-file lambda_event.json)
rm lambda_event.json

status=$(echo "$response" | jq -r '.body' | jq -r '.status')
if [[ "$status" == "healthy" ]]; then
  echo "✅ /health passed"
else
  echo "❌ Unexpected status: $status"
  exit 1
fi


echo "▶️ Lambda /encrypt test"
cat > lambda_event.json <<EOF
{
  "httpMethod": "POST",
  "path": "/encrypt",
  "body": "{\"key\":\"$key\",\"plain_text\":\"$plain_text\"}"
}
EOF
response=$(cargo lambda invoke --data-file lambda_event.json)
rm lambda_event.json

id=$(echo "$response" | jq -r '.body' | jq -r | jq -e -r '.id') || {
  echo "❌ Failed to extract valid ID"
  exit 1
}
echo "✅ /encrypt returned ID: $id"


echo "▶️ Lambda /decrypt test"
cat > lambda_event.json <<EOF
{
  "httpMethod": "GET",
  "path": "/decrypt/$id/$key"
}
EOF
response=$(cargo lambda invoke --data-file lambda_event.json)
rm lambda_event.json

location=$(echo "$response" | jq -r '.headers.location')
actual=$(echo "$location" | sed -E 's|https?://||; s|^www\.||; s|/$||')

if [[ "$actual" == "$expected"* ]]; then
  echo "✅ Redirected to expected domain: $location"
else
  echo "❌ Mismatch: expected $expected, got $actual"
  exit 1
fi
