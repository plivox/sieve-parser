if [[ -z "${RUSTUP_TOOLCHAIN}" || "${RUSTUP_TOOLCHAIN}" == "" ]]; then
	export RUSTUP_TOOLCHAIN=$(rustup toolchain list | grep default | awk '{print $1}')
fi

echo "${RUSTUP_TOOLCHAIN}"

rustup toolchain install "${RUSTUP_TOOLCHAIN}"
rustup component add rust-src clippy rustfmt --toolchain "${RUSTUP_TOOLCHAIN}"
