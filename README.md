# 🚀 LAI (Local Artificial Intelligence)

  A CLI tool in RUST

### ✔ Use a General-purpose reasoning model (i.e. llama3:latest) - codellam etc. not so good

Great for:

* summarization
* history analysis
* explanation
* tasks requiring context-following

### ✔ Code-capable mode (switch to codellama:*)

Use via:

```bash
export OLLAMA_MODEL="codellama:7b"
```
@TODO: add this to .lai.toml


### ✔ CLI (`lai`) reads piped input, merges context correctly, and hits Ollama’s OpenAI-style endpoint

This is the same ergonomics used in lai.sh/AnythingLLM version.

This “swap models on demand” is the core power of using Ollama from your own CLI.

@TODO: ADD 'hot-swapping' as a command flag
---

# 📊 Next Steps: Improving Accuracy Across Use-Cases


## 1. **Context-Boosting Prompt Template**


```text
You are an assistant that *must* use the CONTEXT provided below.

CONTEXT:
"""
{input}
"""

TASK:
{prompt}

Rules:
- Base your answer ONLY on the context unless the task requires outside knowledge.
- If the context is unclear, say so and ask a clarifying question.
- Do not fabricate details not present in the context.
```

This increases grounding and reduces hallucination across all use cases.

---

## 2. **System Instruction**

Adding a system message will stabilize behavior across models:

```rust
{
  "role": "system",
  "content": "You are a concise, context-grounded assistant. You always prioritize analyzing piped input above general knowledge."
}
```

@TODO: continue to refine [system] prompt

---

## 3. **Model-Specific Personality Tuning**

Prompting with different models:

### `llama3:latest`

Best for:

* summarization
* analysis
* natural language
* general-purpose tasks

### `llama3:instruct` or similar (if available)

Even better at following structured tasks.

### `codellama:*`

Use *only* for:

* code generation
* debugging
* templating

### `qwen2.5` (if you install it)

Surprisingly strong reasoning + low verbosity.

---

## 4. **Automatically choose models based on the prompt**

When you’re ready, I can help you add:

**→ Auto-model-selection logic**
(e.g., if the prompt contains “write”, “fix”, “explain this code”, switch to CodeLlama)

**→ Config file: `~/.config/lai/config.toml`**
(set default model per task type)

**→ Flags:**

```
lai --model codellama:7b -p "write a rust function"
lai --general -p "summarize this history"
```

---

## Examples
### 🟦 A. History analysis

Try:

```bash
journalctl -b | lai -p "what errors am i seeing?"
du -h | lai -p "what directories should i investigate?"
git diff | lai -p "summarize this diff"
```

### 🟪 B. Code tasks

Switch to CodeLlama and try:

```bash
cat src/main.rs | lai -p "refactor this into cleaner functions"
lai -p "write a tokio version of this script"
```

### 🟩 C. Shell usage

```bash
history | lai -p "what aliases would make my workflow faster?"
```

### 🟧 D. Debug logs

```bash
dmesg | lai -p "are there disk errors?"
```

---

# For Future Consideration 

🧰 Build an **Auto-Model Router**
📝 Add a **config file** for `lai`
⚡ Add streaming responses
🛠 Improve how context is merged
🔎 Add flags for modes (`--diff`, `--summary`, `--code`, etc.)

## Hard-coded for Ollama (will add config and flag for provider and model changes) 

### Get list of available models
$ ollama list

NAME                                                                                                ID              SIZE      MODIFIED
codellama:7b                                                                                        8fdf8f752f6e    3.8 GB    12 days ago
hf.co/tensorblock/ChatFrame-Uncensored-Instruct-Small-GGUF:Q4_K_M                                   11ea882791f5    4.9 GB    13 days ago
hf.co/DavidAU/Llama-3.2-8X3B-MOE-Dark-Champion-Instruct-uncensored-abliterated-18.4B-GGUF:Q4_K_M    37010ebb4198    11 GB     13 days ago
codellama:latest                                                                                    8fdf8f752f6e    3.8 GB    2 weeks ago
llama3:latest                                                                                       365c0bd3c000    4.7 GB    4 weeks ago
llama3:8b-instruct-q4_K_M                                                                           9b8f3f3385bf    4.9 GB    5 weeks ago
gemma3:4b                                                                                           a2af6cc3eb7f    3.3 GB    5 weeks ago

### Export desired model to env
$ export OLLAMA_MODEL="llama3:latest"

## CLI entrypoints

### Rust (Ollama)

```bash
cargo install --path .
lai -p "how do I use the 'ls' command?"

scripts/lai.sh -p "how do I use the 'ls' command?"

# Plays well with nushell

```bash
~\Mycelium\find\providers> open Hospital_General_Information.csv | where 'State' =~ 'AZ' | first 10 | lai -p "List the hospitals in tucson?" | split row "\n" | explore --index

```
