#!/bin/sh

# sed
# -i : edit in-place
# -b : binary, to preserve Windows' line endings

# Replaces `FOO` with `BAR` while ignoring the VCS sub-directory.
sed -i -b "s/FOO/BAR/g" `grep -l -R --exclude-dir='*/.svn' "FOO" .`
