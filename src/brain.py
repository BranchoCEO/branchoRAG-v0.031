import branchorag

SCAN_PATH = "."
MEMORY_FILE = "brancho_memory.json"

def run_brain():
    print("--- BranchoRAG v0.01: Active ---")

    try:
        # 1. Initialize the RAG system
        rag = branchorag.BranchoRAG()

        # 2. Tell the AI to look at its own project folder
        print("Scanning project folders...")
        rag.scan_folder(SCAN_PATH)
        print(f"  Found {rag.node_count()} file(s).")

        # 3. Save the findings to your SSD
        rag.save_memory(MEMORY_FILE)
        print("✅ Success: BranchoRAG has mapped your project.")

    except Exception as e:
        print(f"❌ BranchoRAG failed: {e}")
        raise

if __name__ == "__main__":
    run_brain()
