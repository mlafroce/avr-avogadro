read -p 'Package version (ex: 1.0-1): ' VERSION

PACKAGE_NAME="avr-avogadro_${VERSION}"

APP=../target/release/avr-avogadro
if [ ! -f "$APP" ]; then
    echo "$APP does not exist, build in parent directory with 'cargo build --release'"
else 
    cp -r deb $PACKAGE_NAME
    mkdir -p $PACKAGE_NAME/usr/local/bin/
    cp -p $APP $PACKAGE_NAME/usr/local/bin/
    dpkg-deb --build $PACKAGE_NAME
    rm -r $PACKAGE_NAME
fi
