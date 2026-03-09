import branchorag
import os

# Your target folder path
SCAN_PATH = r"C:\Users\grint\Documents\OneDrive\roop_row\MONEYMOINE\Web Training"
MEMORY_FILE = "brancho_memory.json"

def run_brain():
    print("--- BranchoRAG v0.02: The Reader ---")

    if not os.path.exists(SCAN_PATH):
        raise FileNotFoundError(f"Path not found: {SCAN_PATH}")

    try:
        rag = branchorag.BranchoRAG()

        print(f"Reading files in: {SCAN_PATH}...")
        rag.scan_folder(SCAN_PATH)

        print(f"  Successfully read {rag.node_count()} file(s).")

        rag.save_memory(MEMORY_FILE)
        print(f"✅ Success: Knowledge saved to {MEMORY_FILE}.")

    except Exception as e:
        print(f"❌ BranchoRAG failed: {e}")
        raise

if __name__ == "__main__":
    run_brain()
