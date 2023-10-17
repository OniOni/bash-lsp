import sys

print("echo started...", file=sys.stderr)

while True:
    msg = sys.stdin.read()

    print(msg, flush=True)

    out = f"Read and wrote: {msg}\n"
    print(out, file=sys.stderr)

    with open("echo_logs.txt") as l:
        l.write()
