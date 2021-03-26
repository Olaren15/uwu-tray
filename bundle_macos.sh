echo "Building uwu-tray MacOS .app package"

mkdir -p assets/macos/uwu-tray.app/Contents/MacOS
mkdir -p assets/macos/uwu-tray.app/Contents/Resources

cp target/release/uwu-tray assets/macos/uwu-tray.app/Contents/MacOS/uwu-tray
cp res/uwu.icns            assets/macos/uwu-tray.app/Contents/Resources/AppIcon.icns
cp res/Info.plist          assets/macos/uwu-tray.app/Contents/Info.plist

hdiutil create assets/uwu-tray.dmg -volname uwu-tray -srcfolder assets/macos -ov
echo "MacOS .app package is built"
