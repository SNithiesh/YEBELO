import json
import sys

def verify(filename):
    print(f"Checking {filename}...")
    valid = 0
    invalid = 0
    with open(filename, 'r', encoding='utf-8') as f:
        for idx, line in enumerate(f, 1):
            line = line.strip()
            if not line:
                continue
            try:
                data = json.loads(line)
                # Check required fields
                if "from" in data and "to" in data and "amount" in data and len(data.keys()) == 3:
                    if isinstance(data["from"], str) and isinstance(data["to"], str) and isinstance(data["amount"], int):
                        valid += 1
                    else:
                        print(f"Line {idx} invalid types: {line}")
                        invalid += 1
                else:
                    print(f"Line {idx} missing or extra keys: {line}")
                    invalid += 1
            except Exception as e:
                print(f"Line {idx} failed JSON parse: {e}")
                invalid += 1

    print(f"Finished: {valid} valid transfers, {invalid} errors.")

if __name__ == "__main__":
    if len(sys.argv) > 1:
        verify(sys.argv[1])
    else:
        verify("output/transfers.jsonl")

