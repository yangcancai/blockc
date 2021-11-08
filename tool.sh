#!/bin/bash
build(){
rustup target add aarch64-apple-ios
rustup target add x86_64-apple-ios
mkdir -p target/universal/release
mkdir -p target/universal/debug
# We need the SDK Root
export SDKROOT=`xcrun --sdk macosx --show-sdk-path`

Member=$1
REL=""
TARGET_OUT=$2
if [ $2 == "release" ];then
	REL=--$2
fi
Path="--manifest-path ${CARGO_MAKE_WORKSPACE_WORKING_DIRECTORY}/native/${Member}/Cargo.toml"
Lib=lib${CARGO_MAKE_CRATE_FS_NAME}.a
# We will build two different frameworks:
# The first one contains X86_64 iOS (Simulator) and ARM iOS
# The other contains Catalyst ARM and Catalyst X86_64
# Then we tell Xcode to only use the first one for iOS builds, 
# and the second one for macOS builds.
# The reason is that fat binaries can't contain the same architecture
# slice twice (i.e. ARM64 iOS and ARM64 Catalyst)

# Simulator:
echo "Building for iOS X86_64 (Simulator)..."
cargo build $Path -Z build-std --target x86_64-apple-ios ${REL}

# X86 Catalyst
# use `cargo build-std` to automatically build the std for non-tier1 platforms
echo "Building for Mac Catalyst X86_64..."
#cargo +nightly build $Path -Z build-std --target x86_64-apple-ios-macabi --release

# ARM64 Catalyst
echo "Building for Mac Catalyst ARM64..."
#cargo +nightly build $Path -Z build-std --target aarch64-apple-ios-macabi --release

# iOS
echo "Building for ARM iOS..."
cargo build $Path -Z build-std --target aarch64-apple-ios ${REL} 


# Build Fat Libraries:
# lipo together the different architectures into a universal 'fat' file

# macOS
echo "Building Fat Libaries"
#lipo -create -output target/universal/$Lib".a" target/{aarch64-apple-ios-macabi,x86_64-apple-ios-macabi}/release/$Lib.a

echo "Wrote target/universal/"

# iOS
lipo -create -output ${CARGO_MAKE_WORKSPACE_WORKING_DIRECTORY}/target/universal/$TARGET_OUT/$Lib ${CARGO_MAKE_WORKSPACE_WORKING_DIRECTORY}/target/{aarch64-apple-ios,x86_64-apple-ios}/$TARGET_OUT/$Lib

 cp ${CARGO_MAKE_WORKSPACE_WORKING_DIRECTORY}/target/universal/$TARGET_OUT/${Lib} \
    ${CARGO_MAKE_WORKSPACE_WORKING_DIRECTORY}/packages/${CARGO_MAKE_CRATE_FS_NAME}/ios/${Lib}
 cp ${CARGO_MAKE_WORKSPACE_WORKING_DIRECTORY}/native/${CARGO_MAKE_CRATE_CURRENT_WORKSPACE_MEMBER}/binding.h \
    ${CARGO_MAKE_WORKSPACE_WORKING_DIRECTORY}/packages/${CARGO_MAKE_CRATE_FS_NAME}/ios/Classes/binding.h
}
run(){
	flutter run --no-sound-null-safety
}
make_dev(){
	cargo make build 
}
make_rel(){
	cargo make build -p release
}
help(){
	echo "sh tool.sh make_dev: cargo make build"
	echo "sh tool.sh make_rel: cargo make build -p release"
	echo "sh tool.sh run: execute flutter run --no-sound-null-safety"
	echo "sh tool.sh build <Member> <rel>"
	echo "Member:"
	echo "native/hb-ffi: hb-ffi"
	echo "rel:"
	echo "release: build release "
	echo "debug: build debug"
}
case $1 in
build) build $2 $3;;
run) run;;
make_dev) make_dev;;
make_rel) make_rel;;
*) help;
esac