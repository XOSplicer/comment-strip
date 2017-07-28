#!/usr/bin/env bash
STRIP="target/release/comment-strip"
set -x
set -e
test -x $STRIP
$STRIP --shell-style "test/shell_test.sh" | diff "test/shell_test.expected.sh" -
$STRIP --xml-style "test/xml_test.xml" | diff "test/xml_test.expected.xml" -
#$STRIP --c-style "test/c_test.c" | diff "test/c_test.expected.c" -
