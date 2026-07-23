#!/bin/bash
# *****************************************************************************
# :  Lumtract  Lumtract/Git 
# : ./rename.sh
# : 
# *****************************************************************************

set -e  # 

# 
OLD="Lumtract"
NEW="Lumtract"
# /
OLD_LOWER="lumtract"
NEW_LOWER="lumtract"

echo " : ${OLD}  ${NEW}"

# 1. 
echo " ..."
find . -type f \
    -not -path "./.git/*" \
    -not -path "./target/*" \
    -not -path "./node_modules/*" \
    -not -path "./.next/*" \
    -not -path "./dist/*" \
    -not -path "./build/*" \
    -not -path "*/__pycache__/*" \
    -not -name "*.png" \
    -not -name "*.jpg" \
    -not -name "*.ico" \
    -not -name "*.lock" \
    -not -name "*.svg" \
    -not -name "*.woff2" \
    -exec sed -i '' "s/${OLD}/${NEW}/g" {} + \
    -exec sed -i '' "s/${OLD_LOWER}/${NEW_LOWER}/g" {} +

# 2.  /
echo " ..."
find . -depth \
    -not -path "./.git/*" \
    -not -path "./target/*" \
    -not -path "./node_modules/*" \
    -not -path "./.next/*" \
    -not -path "./dist/*" \
    -name "*${OLD}*" -o -name "*${OLD_LOWER}*" | while read -r path; do
    dir=$(dirname "$path")
    base=$(basename "$path")
    new_base=$(echo "$base" | sed "s/${OLD}/${NEW}/g" | sed "s/${OLD_LOWER}/${NEW_LOWER}/g")
    if [ "$base" != "$new_base" ]; then
        echo "  : $path  $dir/$new_base"
        mv "$path" "$dir/$new_base"
    fi
done

# 3.  Cargo.lock  package-lock
echo " ..."
rm -rf target
rm -rf .next
rm -rf node_modules
rm -f Cargo.lock
rm -f package-lock.json

# 4.  & 
echo " ..."
#  (Rust)
if [ -f "dag-generator/Cargo.toml" ]; then
    cd dag-generator
    cargo update
    cd ..
fi

#  (Node)
if [ -f "web-viewer/package.json" ]; then
    cd web-viewer
    npm install
    cd ..
fi

# 5. 
echo " ..."
if cargo test --workspace -- --nocapture; then
    echo ""
    echo " "
    echo " : git add . && git commit -m 'chore: rename ${OLD}  ${NEW}'"
else
    echo ""
    echo " "
    exit 1
fi
