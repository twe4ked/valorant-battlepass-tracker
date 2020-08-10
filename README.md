# Valorant Battlepass XP

    cargo watch -i .gitignore -i "static/*" -s "wasm-pack build --target web --out-name wasm --out-dir ./static"
    miniserve ./static --index index.html

## Releasing

    git add -f static/wasm*
    git commit -m "Release"
    git push origin `git subtree split --prefix static`:gh-pages --force
    git reset HEAD~
