import branchorag
import os
import time

# Your target folder path
SCAN_PATH = r"C:\Users\grint\Documents\OneDrive\roop_row\MONEYMOINE\Web Training"
MEMORY_FILE = "brancho_memory.json"

def run_brain():
    print("--- BranchoRAG v0.02: The Reader ---")

    if not os.path.isdir(SCAN_PATH):
        raise FileNotFoundError(f"Scan path not found or is not a folder: {SCAN_PATH}")

    try:
        rag = branchorag.BranchoRAG()

        print(f"Reading files in: {SCAN_PATH}...")
        start = time.perf_counter()
        rag.scan_folder(SCAN_PATH)
        elapsed = time.perf_counter() - start

        print(f"  Successfully read {rag.node_count()} file(s) in {elapsed:.2f}s.")

        rag.save_memory(MEMORY_FILE)
        print(f"✅ Success: Knowledge saved to {MEMORY_FILE}.")

    except Exception as e:
        print(f"❌ BranchoRAG failed: {e}")
        raise

if __name__ == "__main__":
    run_brain()
