#!/usr/bin/bash

# Generate .po files for Dynamic Permissions

xgettext \
	src/definitions/consent.rs \
	--package-name="portable-config" \
	--package-version="0.1.0" \
	--copyright-holder="Kimiblock Moe, Kraftland Organisation Founder" \
	--language=Rust \
	--from-code=UTF-8 \
	--output-dir po

for file in $(find po/ -maxdepth 1 -type f); do
	if [[ "${file}" = "po/messages.po" ]]; then
		continue
	fi

	msgmerge \
		--update \
		"${file}" \
		po/messages.po
done