import json

def format_jsonl(input_file, output_file):
    count = 0
    # Try utf-16-le (PowerShell default redirect encoding) then utf-8
    content = None
    for enc in ['utf-16', 'utf-16-le', 'utf-8', 'latin-1']:
        try:
            with open(input_file, 'r', encoding=enc) as f:
                content = f.readlines()
            print(f"Successfully read input with {enc} encoding.")
            break
        except Exception:
            continue

    if not content:
        print("Failed to decode input file")
        return

    with open(output_file, 'w', encoding='utf-8') as outfile:
        for line in content:
            line = line.strip()
            if not line:
                continue
            try:
                doc = json.loads(line)
                transfers = doc.get("@data", {}).get("transfers", [])
                for t in transfers:
                    record = {
                        "from": t["from"],
                        "to": t["to"],
                        "amount": int(t["amount"])
                    }
                    outfile.write(json.dumps(record) + "\n")
                    count += 1
            except Exception as e:
                pass
    print(f"Extracted and formatted {count} transfers successfully!")

if __name__ == "__main__":
    format_jsonl("output/transfers_raw.jsonl", "output/transfers.jsonl")
