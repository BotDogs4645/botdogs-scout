#!/bin/bash

gnome-terminal -- bash -c "npx tailwindcss -i ./style/main.scss -o ./target/site/pkg/botdogs-scout.css --watch; exec bash"

cargo leptos watch