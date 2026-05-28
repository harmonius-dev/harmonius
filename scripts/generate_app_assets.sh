#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
ASSET_ROOT="${ROOT}/app/Assets.xcassets"
APP_ICON="${ASSET_ROOT}/AppIcon.appiconset"
ICON_SOURCE="${ROOT}/app/AssetSources/AppIcon.svg"
LAUNCH_BACKGROUND="${ASSET_ROOT}/LaunchBackground.colorset"

if [[ ! -f "${ICON_SOURCE}" ]]; then
  echo "error: missing app icon source at ${ICON_SOURCE}" >&2
  exit 1
fi

rm -rf "${ASSET_ROOT}"
mkdir -p "${APP_ICON}" "${LAUNCH_BACKGROUND}"

cat > "${ASSET_ROOT}/Contents.json" <<'JSON'
{
  "info": {
    "author": "xcode",
    "version": 1
  }
}
JSON

cat > "${LAUNCH_BACKGROUND}/Contents.json" <<'JSON'
{
  "colors": [
    {
      "color": {
        "color-space": "srgb",
        "components": {
          "alpha": "1.000",
          "blue": "0.180",
          "green": "0.102",
          "red": "0.071"
        }
      },
      "idiom": "universal"
    }
  ],
  "info": {
    "author": "xcode",
    "version": 1
  }
}
JSON

if command -v magick >/dev/null 2>&1; then
  MAGICK=(magick)
elif command -v convert >/dev/null 2>&1; then
  MAGICK=(convert)
else
  echo "error: ImageMagick is required to generate app assets" >&2
  exit 1
fi

render_icon() {
  local filename="$1"
  local pixels="$2"

  "${MAGICK[@]}" \
    -background none \
    "${ICON_SOURCE}" \
    -resize "${pixels}x${pixels}" \
    -gravity center \
    -extent "${pixels}x${pixels}" \
    "PNG32:${APP_ICON}/${filename}"
}

render_icon "iphone-20x20@2x.png" 40
render_icon "iphone-20x20@3x.png" 60
render_icon "iphone-29x29@2x.png" 58
render_icon "iphone-29x29@3x.png" 87
render_icon "iphone-40x40@2x.png" 80
render_icon "iphone-40x40@3x.png" 120
render_icon "iphone-60x60@2x.png" 120
render_icon "iphone-60x60@3x.png" 180
render_icon "ipad-20x20@1x.png" 20
render_icon "ipad-20x20@2x.png" 40
render_icon "ipad-29x29@1x.png" 29
render_icon "ipad-29x29@2x.png" 58
render_icon "ipad-40x40@1x.png" 40
render_icon "ipad-40x40@2x.png" 80
render_icon "ipad-76x76@1x.png" 76
render_icon "ipad-76x76@2x.png" 152
render_icon "ipad-83_5x83_5@2x.png" 167
render_icon "ios-marketing-1024x1024@1x.png" 1024

cat > "${APP_ICON}/Contents.json" <<'JSON'
{
  "images": [
    {
      "filename": "iphone-20x20@2x.png",
      "idiom": "iphone",
      "scale": "2x",
      "size": "20x20"
    },
    {
      "filename": "iphone-20x20@3x.png",
      "idiom": "iphone",
      "scale": "3x",
      "size": "20x20"
    },
    {
      "filename": "iphone-29x29@2x.png",
      "idiom": "iphone",
      "scale": "2x",
      "size": "29x29"
    },
    {
      "filename": "iphone-29x29@3x.png",
      "idiom": "iphone",
      "scale": "3x",
      "size": "29x29"
    },
    {
      "filename": "iphone-40x40@2x.png",
      "idiom": "iphone",
      "scale": "2x",
      "size": "40x40"
    },
    {
      "filename": "iphone-40x40@3x.png",
      "idiom": "iphone",
      "scale": "3x",
      "size": "40x40"
    },
    {
      "filename": "iphone-60x60@2x.png",
      "idiom": "iphone",
      "scale": "2x",
      "size": "60x60"
    },
    {
      "filename": "iphone-60x60@3x.png",
      "idiom": "iphone",
      "scale": "3x",
      "size": "60x60"
    },
    {
      "filename": "ipad-20x20@1x.png",
      "idiom": "ipad",
      "scale": "1x",
      "size": "20x20"
    },
    {
      "filename": "ipad-20x20@2x.png",
      "idiom": "ipad",
      "scale": "2x",
      "size": "20x20"
    },
    {
      "filename": "ipad-29x29@1x.png",
      "idiom": "ipad",
      "scale": "1x",
      "size": "29x29"
    },
    {
      "filename": "ipad-29x29@2x.png",
      "idiom": "ipad",
      "scale": "2x",
      "size": "29x29"
    },
    {
      "filename": "ipad-40x40@1x.png",
      "idiom": "ipad",
      "scale": "1x",
      "size": "40x40"
    },
    {
      "filename": "ipad-40x40@2x.png",
      "idiom": "ipad",
      "scale": "2x",
      "size": "40x40"
    },
    {
      "filename": "ipad-76x76@1x.png",
      "idiom": "ipad",
      "scale": "1x",
      "size": "76x76"
    },
    {
      "filename": "ipad-76x76@2x.png",
      "idiom": "ipad",
      "scale": "2x",
      "size": "76x76"
    },
    {
      "filename": "ipad-83_5x83_5@2x.png",
      "idiom": "ipad",
      "scale": "2x",
      "size": "83.5x83.5"
    },
    {
      "filename": "ios-marketing-1024x1024@1x.png",
      "idiom": "ios-marketing",
      "scale": "1x",
      "size": "1024x1024"
    }
  ],
  "info": {
    "author": "xcode",
    "version": 1
  }
}
JSON
