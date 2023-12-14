#!/bin/bash

npx tailwindcss -i ./style/main.scss -o ./target/site/pkg/botdogs-scout.css --watch &

cargo leptos watch