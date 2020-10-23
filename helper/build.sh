#! /bin/bash -eu

PROJ_PATH=$(cd "$(dirname "$(readlink -f $0)")" && pwd)
cd "${PROJ_PATH}/.."

mkdir -p artifacts

docker build -f 'Dockerfile'
docker run --rm -v "${PWD}/artifacts:/artifacts" --entrypoint="cp" KeroKeroRepeat "/KeroKeroRepeat" "/artifacts"
