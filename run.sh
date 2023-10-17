set -eo pipefail

cargo build
python -m src.client
