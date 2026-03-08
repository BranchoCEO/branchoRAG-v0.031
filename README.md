Gemini said
Since you’ve officially rebranded to BranchoRAG, your README should reflect that this isn't just a basic script—it’s a Local Retrieval-Augmented Generation engine. This file is what makes your GitHub look professional and tells other developers (and your future self) how to use the "Brain."

Create a new file in your main folder named README.md and paste this in:

🌿 BranchoRAG v0.01
BranchoRAG is a high-performance, local AI engine built to map, understand, and retrieve code logic from your local machine. By combining the raw speed of Rust with the deep learning power of PyTorch, BranchoRAG creates a "living memory" of your project structure.

🚀 The Core Tech
The Muscle (Rust): Scans the file system at lightning speeds using walkdir and builds the code graph.

The Bridge (Maturin/PyO3): Seamlessly connects Rust performance to Python flexibility.

The Brain (PyTorch GNN): Processes file relationships using Graph Neural Networks to understand how your project "hangs together."

📂 Project Structure
src/lib.rs: The Rust-powered scanning and graph-building engine.

src/brain.py: The Python neural network and main entry point.

brancho_memory.json: The persistent "Long-Term Memory" of your AI.

🛠️ Installation & Setup
Create your environment:

Bash
python -m venv .venv
.\.venv\Scripts\activate
Install dependencies:

Bash
pip install maturin torch torch-geometric
Build the Rust Engine:

Bash
maturin develop
Boot the Brain:

Bash
python src/brain.py
🧠 Current Capabilities (v0.01)
[x] Automatic File Discovery: Rust-powered recursive folder scanning.

[x] Persistent Memory: Graphs are saved to SSD and can be reloaded without re-scanning.

[x] Neural Mapping: Converts file structures into N-dimensional tensors for AI processing.

🗺️ Roadmap
[ ] v0.02 (Vector Embeddings): Reading actual code text to understand meaning.

[ ] v0.03 (Natural Language Interface): Chatting with your local files to generate code.
