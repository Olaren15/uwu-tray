echo "Building uwu-tray MacOS .app package"

if [ -d "assets/macos/uwu-tray.app/Contents/MacOS" ];
then
  echo "[assets/macos/uwu-tray.app/Contents/MacOS] already exists"
else
  mkdir -p assets/macos/uwu-tray.app/Contents/MacOS
fi

if [ -d "assets/macos/uwu-tray.app/Contents/Resources" ];
then
  echo "[assets/macos/uwu-tray.app/Contents/Resources] already exists"
else
  mkdir -p assets/macos/uwu-tray.app/Contents/Resources
fi

cp target/release/uwu-tray assets/macos/uwu-tray.app/Contents/MacOS/uwu-tray
cp res/uwu.icns            assets/macos/uwu-tray.app/Contents/Resources/AppIcon.icns
cp res/Info.plist          assets/macos/uwu-tray.app/Contents/Info.plist

if type hdiutil >/dev/null 2>&1;
then
  hdiutil create assets/uwu-tray.dmg -volname uwu-tray -srcfolder assets/macos -ov
  echo "MacOS .app package is built"
else
  echo "Require [hdiutil] to build MacOS .app package. Aborting"
fi
