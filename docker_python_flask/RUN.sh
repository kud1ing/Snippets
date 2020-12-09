#!/bin/sh

# -it   : interactive/tty
# -name : container name
# -rm   : remove container after exit
# -p    : publish ports
docker run -it --rm -p 8000:8000 --name test-docker-python test-docker-python
