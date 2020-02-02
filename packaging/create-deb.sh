read -p 'Package version (ex: 1.0-1): ' VERSION

PACKAGE_NAME="avr-avogadro_${VERSION}"

APP_NAME=avr-avogadro
BUILT_APP=../target/release/$APP_NAME

if [ ! -f "$BUILT_APP" ]; then
    echo "$BUILT_APP does not exist, build in parent directory with 'cargo build --release'"
else 
    cp -r deb $PACKAGE_NAME
    sed -i s_\${VERSION}_${VERSION}_g $PACKAGE_NAME/DEBIAN/control
    mkdir -p $PACKAGE_NAME/usr/bin/
    mkdir -p $PACKAGE_NAME/usr/share/applications/
    mkdir -p $PACKAGE_NAME/usr/share/pixmaps/
    cp deb/$APP_NAME.desktop $PACKAGE_NAME/usr/share/applications/
    cp logo.png $PACKAGE_NAME/usr/share/pixmaps/$APP_NAME.png
    objcopy --strip-debug --strip-unneeded $BUILT_APP $PACKAGE_NAME/usr/bin/$APP_NAME
    chmod -R 755 $PACKAGE_NAME 
    fakeroot dpkg-deb --build $PACKAGE_NAME
    rm -r $PACKAGE_NAME
fi
