# Search/replace using `sed`

`sed` arguments:
* `-i`: edit in-place
* `-b`: binary, to preserve Windows' line endings

## Examples

* Replacing `FOO` with `BAR` while ignoring the VCS sub-directory:
  *     sed -i -b "s/FOO/BAR/g" `grep -l -R --exclude-dir='*/.svn' "FOO" .`
    
