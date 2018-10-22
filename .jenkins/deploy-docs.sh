#!/usr/bin/env bash

set -e +x

scp docs.tar.gz "$DOCS_SCP_UPLOAD_TARGET"
curl -X POST "$TOBY_URL" -H "Authorization: Token $TOBY_AUTHORIZATION"
