package hedera

// #cgo CFLAGS: -g -Wall
// #include "native/hedera.h"
// #cgo linux,amd64 LDFLAGS: ${SRCDIR}/native/linux/libhedera.a -ldl -lm
// #cgo darwin,amd64 LDFLAGS: ${SRCDIR}/native/macos/amd64/libhedera.a -ldl -lm
// #cgo darwin,arm64 LDFLAGS: ${SRCDIR}/native/macos/arm64/libhedera.a -ldl -lm
// #cgo windows,amd64 LDFLAGS: ${SRCDIR}/native/windows/libhedera.a -lm
import "C"
