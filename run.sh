BUILD_NAME="uefi-donut-rs"
BUILD_ARCH="x86_64"
BUILD_TYPE="release"

BUILD_FIRMWARE_PATH="/usr/share/OVMF"
BUILD_FIRMWARE_CODE="OVMF_CODE_4M.fd"
BUILD_FIRMWARE_VARS="OVMF_CODE_4M.fd"

cargo build --target x86_64-unknown-uefi --${BUILD_TYPE}

mkdir -p esp/efi/boot
cp target/${BUILD_ARCH}-unknown-uefi/${BUILD_TYPE}/${BUILD_NAME}.efi esp/efi/boot/bootx64.efi

qemu-system-x86_64 -enable-kvm \
    -drive if=pflash,format=raw,readonly=on,file=${BUILD_FIRMWARE_PATH}/${BUILD_FIRMWARE_CODE} \
    -drive if=pflash,format=raw,readonly=on,file=${BUILD_FIRMWARE_PATH}/${BUILD_FIRMWARE_VARS} \
    -drive format=raw,file=fat:rw:esp