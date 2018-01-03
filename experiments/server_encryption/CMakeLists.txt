cmake_minimum_required(VERSION 3.0)

project(server_encryption_test C)

## Set standard compiler options
add_compile_options(
   -std=gnu99 # C standard to use
   -O2        # optimization level
   -g         # enable debug information
# Warnings - Enable
   -Werror
   -Wall
   -Wextra
   -Wbad-function-cast
   -Wpointer-arith
# Warnings - Disable
   #-Wno-deprecated-declarations
   # -Wno-unused-parameter
# Compiler Flags
   #-funsigned-char
   #-fgnu89-inline
   # -fno-omit-frame-pointer # to get nicer stack traces with ASan
   # -fno-optimize-sibling-calls # disable TCO for stack traces
)

set(SOURCES encrypt_decrypt.c)

add_executable(srv_enc_test ${SOURCES})