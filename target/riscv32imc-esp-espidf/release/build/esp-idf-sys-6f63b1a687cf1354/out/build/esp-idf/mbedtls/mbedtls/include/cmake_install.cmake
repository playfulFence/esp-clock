# Install script for directory: /Users/playfulfence/.espressif/esp-idf/release-v4.4/components/mbedtls/mbedtls/include

# Set the install prefix
if(NOT DEFINED CMAKE_INSTALL_PREFIX)
  set(CMAKE_INSTALL_PREFIX "/Users/playfulfence/esp/esp-clock/target/riscv32imc-esp-espidf/release/build/esp-idf-sys-6f63b1a687cf1354/out")
endif()
string(REGEX REPLACE "/$" "" CMAKE_INSTALL_PREFIX "${CMAKE_INSTALL_PREFIX}")

# Set the install configuration name.
if(NOT DEFINED CMAKE_INSTALL_CONFIG_NAME)
  if(BUILD_TYPE)
    string(REGEX REPLACE "^[^A-Za-z0-9_]+" ""
           CMAKE_INSTALL_CONFIG_NAME "${BUILD_TYPE}")
  else()
    set(CMAKE_INSTALL_CONFIG_NAME "")
  endif()
  message(STATUS "Install configuration: \"${CMAKE_INSTALL_CONFIG_NAME}\"")
endif()

# Set the component getting installed.
if(NOT CMAKE_INSTALL_COMPONENT)
  if(COMPONENT)
    message(STATUS "Install component: \"${COMPONENT}\"")
    set(CMAKE_INSTALL_COMPONENT "${COMPONENT}")
  else()
    set(CMAKE_INSTALL_COMPONENT)
  endif()
endif()

# Is this installation the result of a crosscompile?
if(NOT DEFINED CMAKE_CROSSCOMPILING)
  set(CMAKE_CROSSCOMPILING "TRUE")
endif()

# Set default install directory permissions.
if(NOT DEFINED CMAKE_OBJDUMP)
  set(CMAKE_OBJDUMP "/Users/playfulfence/.espressif/tools/riscv32-esp-elf/esp-2021r2-patch3-8.4.0/riscv32-esp-elf/bin/riscv32-esp-elf-objdump")
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xUnspecifiedx" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/include/mbedtls" TYPE FILE PERMISSIONS OWNER_READ OWNER_WRITE GROUP_READ WORLD_READ FILES
    "/Users/playfulfence/.espressif/esp-idf/release-v4.4/components/mbedtls/mbedtls/include/mbedtls/aes.h"
    "/Users/playfulfence/.espressif/esp-idf/release-v4.4/components/mbedtls/mbedtls/include/mbedtls/aesni.h"
    "/Users/playfulfence/.espressif/esp-idf/release-v4.4/components/mbedtls/mbedtls/include/mbedtls/arc4.h"
    "/Users/playfulfence/.espressif/esp-idf/release-v4.4/components/mbedtls/mbedtls/include/mbedtls/aria.h"
    "/Users/playfulfence/.espressif/esp-idf/release-v4.4/components/mbedtls/mbedtls/include/mbedtls/asn1.h"
    "/Users/playfulfence/.espressif/esp-idf/release-v4.4/components/mbedtls/mbedtls/include/mbedtls/asn1write.h"
    "/Users/playfulfence/.espressif/esp-idf/release-v4.4/components/mbedtls/mbedtls/include/mbedtls/base64.h"
    "/Users/playfulfence/.espressif/esp-idf/release-v4.4/components/mbedtls/mbedtls/include/mbedtls/bignum.h"
    "/Users/playfulfence/.espressif/esp-idf/release-v4.4/components/mbedtls/mbedtls/include/mbedtls/blowfish.h"
    "/Users/playfulfence/.espressif/esp-idf/release-v4.4/components/mbedtls/mbedtls/include/mbedtls/bn_mul.h"
    "/Users/playfulfence/.espressif/esp-idf/release-v4.4/components/mbedtls/mbedtls/include/mbedtls/camellia.h"
    "/Users/playfulfence/.espressif/esp-idf/release-v4.4/components/mbedtls/mbedtls/include/mbedtls/ccm.h"
    "/Users/playfulfence/.espressif/esp-idf/release-v4.4/components/mbedtls/mbedtls/include/mbedtls/certs.h"
    "/Users/playfulfence/.espressif/esp-idf/release-v4.4/components/mbedtls/mbedtls/include/mbedtls/chacha20.h"
    "/Users/playfulfence/.espressif/esp-idf/release-v4.4/components/mbedtls/mbedtls/include/mbedtls/chachapoly.h"
    "/Users/playfulfence/.espressif/esp-idf/release-v4.4/components/mbedtls/mbedtls/include/mbedtls/check_config.h"
    "/Users/playfulfence/.espressif/esp-idf/release-v4.4/components/mbedtls/mbedtls/include/mbedtls/cipher.h"
    "/Users/playfulfence/.espressif/esp-idf/release-v4.4/components/mbedtls/mbedtls/include/mbedtls/cipher_internal.h"
    "/Users/playfulfence/.espressif/esp-idf/release-v4.4/components/mbedtls/mbedtls/include/mbedtls/cmac.h"
    "/Users/playfulfence/.espressif/esp-idf/release-v4.4/components/mbedtls/mbedtls/include/mbedtls/compat-1.3.h"
    "/Users/playfulfence/.espressif/esp-idf/release-v4.4/components/mbedtls/mbedtls/include/mbedtls/config.h"
    "/Users/playfulfence/.espressif/esp-idf/release-v4.4/components/mbedtls/mbedtls/include/mbedtls/config_psa.h"
    "/Users/playfulfence/.espressif/esp-idf/release-v4.4/components/mbedtls/mbedtls/include/mbedtls/constant_time.h"
    "/Users/playfulfence/.espressif/esp-idf/release-v4.4/components/mbedtls/mbedtls/include/mbedtls/ctr_drbg.h"
    "/Users/playfulfence/.espressif/esp-idf/release-v4.4/components/mbedtls/mbedtls/include/mbedtls/debug.h"
    "/Users/playfulfence/.espressif/esp-idf/release-v4.4/components/mbedtls/mbedtls/include/mbedtls/des.h"
    "/Users/playfulfence/.espressif/esp-idf/release-v4.4/components/mbedtls/mbedtls/include/mbedtls/dhm.h"
    "/Users/playfulfence/.espressif/esp-idf/release-v4.4/components/mbedtls/mbedtls/include/mbedtls/ecdh.h"
    "/Users/playfulfence/.espressif/esp-idf/release-v4.4/components/mbedtls/mbedtls/include/mbedtls/ecdsa.h"
    "/Users/playfulfence/.espressif/esp-idf/release-v4.4/components/mbedtls/mbedtls/include/mbedtls/ecjpake.h"
    "/Users/playfulfence/.espressif/esp-idf/release-v4.4/components/mbedtls/mbedtls/include/mbedtls/ecp.h"
    "/Users/playfulfence/.espressif/esp-idf/release-v4.4/components/mbedtls/mbedtls/include/mbedtls/ecp_internal.h"
    "/Users/playfulfence/.espressif/esp-idf/release-v4.4/components/mbedtls/mbedtls/include/mbedtls/entropy.h"
    "/Users/playfulfence/.espressif/esp-idf/release-v4.4/components/mbedtls/mbedtls/include/mbedtls/entropy_poll.h"
    "/Users/playfulfence/.espressif/esp-idf/release-v4.4/components/mbedtls/mbedtls/include/mbedtls/error.h"
    "/Users/playfulfence/.espressif/esp-idf/release-v4.4/components/mbedtls/mbedtls/include/mbedtls/gcm.h"
    "/Users/playfulfence/.espressif/esp-idf/release-v4.4/components/mbedtls/mbedtls/include/mbedtls/havege.h"
    "/Users/playfulfence/.espressif/esp-idf/release-v4.4/components/mbedtls/mbedtls/include/mbedtls/hkdf.h"
    "/Users/playfulfence/.espressif/esp-idf/release-v4.4/components/mbedtls/mbedtls/include/mbedtls/hmac_drbg.h"
    "/Users/playfulfence/.espressif/esp-idf/release-v4.4/components/mbedtls/mbedtls/include/mbedtls/md.h"
    "/Users/playfulfence/.espressif/esp-idf/release-v4.4/components/mbedtls/mbedtls/include/mbedtls/md2.h"
    "/Users/playfulfence/.espressif/esp-idf/release-v4.4/components/mbedtls/mbedtls/include/mbedtls/md4.h"
    "/Users/playfulfence/.espressif/esp-idf/release-v4.4/components/mbedtls/mbedtls/include/mbedtls/md5.h"
    "/Users/playfulfence/.espressif/esp-idf/release-v4.4/components/mbedtls/mbedtls/include/mbedtls/md_internal.h"
    "/Users/playfulfence/.espressif/esp-idf/release-v4.4/components/mbedtls/mbedtls/include/mbedtls/memory_buffer_alloc.h"
    "/Users/playfulfence/.espressif/esp-idf/release-v4.4/components/mbedtls/mbedtls/include/mbedtls/net.h"
    "/Users/playfulfence/.espressif/esp-idf/release-v4.4/components/mbedtls/mbedtls/include/mbedtls/net_sockets.h"
    "/Users/playfulfence/.espressif/esp-idf/release-v4.4/components/mbedtls/mbedtls/include/mbedtls/nist_kw.h"
    "/Users/playfulfence/.espressif/esp-idf/release-v4.4/components/mbedtls/mbedtls/include/mbedtls/oid.h"
    "/Users/playfulfence/.espressif/esp-idf/release-v4.4/components/mbedtls/mbedtls/include/mbedtls/padlock.h"
    "/Users/playfulfence/.espressif/esp-idf/release-v4.4/components/mbedtls/mbedtls/include/mbedtls/pem.h"
    "/Users/playfulfence/.espressif/esp-idf/release-v4.4/components/mbedtls/mbedtls/include/mbedtls/pk.h"
    "/Users/playfulfence/.espressif/esp-idf/release-v4.4/components/mbedtls/mbedtls/include/mbedtls/pk_internal.h"
    "/Users/playfulfence/.espressif/esp-idf/release-v4.4/components/mbedtls/mbedtls/include/mbedtls/pkcs11.h"
    "/Users/playfulfence/.espressif/esp-idf/release-v4.4/components/mbedtls/mbedtls/include/mbedtls/pkcs12.h"
    "/Users/playfulfence/.espressif/esp-idf/release-v4.4/components/mbedtls/mbedtls/include/mbedtls/pkcs5.h"
    "/Users/playfulfence/.espressif/esp-idf/release-v4.4/components/mbedtls/mbedtls/include/mbedtls/platform.h"
    "/Users/playfulfence/.espressif/esp-idf/release-v4.4/components/mbedtls/mbedtls/include/mbedtls/platform_time.h"
    "/Users/playfulfence/.espressif/esp-idf/release-v4.4/components/mbedtls/mbedtls/include/mbedtls/platform_util.h"
    "/Users/playfulfence/.espressif/esp-idf/release-v4.4/components/mbedtls/mbedtls/include/mbedtls/poly1305.h"
    "/Users/playfulfence/.espressif/esp-idf/release-v4.4/components/mbedtls/mbedtls/include/mbedtls/psa_util.h"
    "/Users/playfulfence/.espressif/esp-idf/release-v4.4/components/mbedtls/mbedtls/include/mbedtls/ripemd160.h"
    "/Users/playfulfence/.espressif/esp-idf/release-v4.4/components/mbedtls/mbedtls/include/mbedtls/rsa.h"
    "/Users/playfulfence/.espressif/esp-idf/release-v4.4/components/mbedtls/mbedtls/include/mbedtls/rsa_internal.h"
    "/Users/playfulfence/.espressif/esp-idf/release-v4.4/components/mbedtls/mbedtls/include/mbedtls/sha1.h"
    "/Users/playfulfence/.espressif/esp-idf/release-v4.4/components/mbedtls/mbedtls/include/mbedtls/sha256.h"
    "/Users/playfulfence/.espressif/esp-idf/release-v4.4/components/mbedtls/mbedtls/include/mbedtls/sha512.h"
    "/Users/playfulfence/.espressif/esp-idf/release-v4.4/components/mbedtls/mbedtls/include/mbedtls/ssl.h"
    "/Users/playfulfence/.espressif/esp-idf/release-v4.4/components/mbedtls/mbedtls/include/mbedtls/ssl_cache.h"
    "/Users/playfulfence/.espressif/esp-idf/release-v4.4/components/mbedtls/mbedtls/include/mbedtls/ssl_ciphersuites.h"
    "/Users/playfulfence/.espressif/esp-idf/release-v4.4/components/mbedtls/mbedtls/include/mbedtls/ssl_cookie.h"
    "/Users/playfulfence/.espressif/esp-idf/release-v4.4/components/mbedtls/mbedtls/include/mbedtls/ssl_internal.h"
    "/Users/playfulfence/.espressif/esp-idf/release-v4.4/components/mbedtls/mbedtls/include/mbedtls/ssl_ticket.h"
    "/Users/playfulfence/.espressif/esp-idf/release-v4.4/components/mbedtls/mbedtls/include/mbedtls/threading.h"
    "/Users/playfulfence/.espressif/esp-idf/release-v4.4/components/mbedtls/mbedtls/include/mbedtls/timing.h"
    "/Users/playfulfence/.espressif/esp-idf/release-v4.4/components/mbedtls/mbedtls/include/mbedtls/version.h"
    "/Users/playfulfence/.espressif/esp-idf/release-v4.4/components/mbedtls/mbedtls/include/mbedtls/x509.h"
    "/Users/playfulfence/.espressif/esp-idf/release-v4.4/components/mbedtls/mbedtls/include/mbedtls/x509_crl.h"
    "/Users/playfulfence/.espressif/esp-idf/release-v4.4/components/mbedtls/mbedtls/include/mbedtls/x509_crt.h"
    "/Users/playfulfence/.espressif/esp-idf/release-v4.4/components/mbedtls/mbedtls/include/mbedtls/x509_csr.h"
    "/Users/playfulfence/.espressif/esp-idf/release-v4.4/components/mbedtls/mbedtls/include/mbedtls/xtea.h"
    )
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xUnspecifiedx" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/include/psa" TYPE FILE PERMISSIONS OWNER_READ OWNER_WRITE GROUP_READ WORLD_READ FILES
    "/Users/playfulfence/.espressif/esp-idf/release-v4.4/components/mbedtls/mbedtls/include/psa/crypto.h"
    "/Users/playfulfence/.espressif/esp-idf/release-v4.4/components/mbedtls/mbedtls/include/psa/crypto_builtin_composites.h"
    "/Users/playfulfence/.espressif/esp-idf/release-v4.4/components/mbedtls/mbedtls/include/psa/crypto_builtin_primitives.h"
    "/Users/playfulfence/.espressif/esp-idf/release-v4.4/components/mbedtls/mbedtls/include/psa/crypto_compat.h"
    "/Users/playfulfence/.espressif/esp-idf/release-v4.4/components/mbedtls/mbedtls/include/psa/crypto_config.h"
    "/Users/playfulfence/.espressif/esp-idf/release-v4.4/components/mbedtls/mbedtls/include/psa/crypto_driver_common.h"
    "/Users/playfulfence/.espressif/esp-idf/release-v4.4/components/mbedtls/mbedtls/include/psa/crypto_driver_contexts_composites.h"
    "/Users/playfulfence/.espressif/esp-idf/release-v4.4/components/mbedtls/mbedtls/include/psa/crypto_driver_contexts_primitives.h"
    "/Users/playfulfence/.espressif/esp-idf/release-v4.4/components/mbedtls/mbedtls/include/psa/crypto_extra.h"
    "/Users/playfulfence/.espressif/esp-idf/release-v4.4/components/mbedtls/mbedtls/include/psa/crypto_platform.h"
    "/Users/playfulfence/.espressif/esp-idf/release-v4.4/components/mbedtls/mbedtls/include/psa/crypto_se_driver.h"
    "/Users/playfulfence/.espressif/esp-idf/release-v4.4/components/mbedtls/mbedtls/include/psa/crypto_sizes.h"
    "/Users/playfulfence/.espressif/esp-idf/release-v4.4/components/mbedtls/mbedtls/include/psa/crypto_struct.h"
    "/Users/playfulfence/.espressif/esp-idf/release-v4.4/components/mbedtls/mbedtls/include/psa/crypto_types.h"
    "/Users/playfulfence/.espressif/esp-idf/release-v4.4/components/mbedtls/mbedtls/include/psa/crypto_values.h"
    )
endif()

