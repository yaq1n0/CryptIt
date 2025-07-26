<template>
  <main class="container">
    <h1>CryptIt - File Encryption with Secret Sharing</h1>
    
    <div class="section">
      <h2>File Selection</h2>
      <div class="file-input">
        <button @click="selectFile" class="btn btn-secondary">Select File</button>
        <span class="file-path">{{ selectedFile || "No file selected" }}</span>
      </div>
      
      <div class="file-input">
        <button @click="selectOutputDir" class="btn btn-secondary">Select Output Directory</button>
        <span class="file-path">{{ outputDir || "No directory selected" }}</span>
      </div>
    </div>

    <div class="section">
      <h2>Shamir Secret Sharing Configuration</h2>
      <div class="sss-config">
        <div class="input-group">
          <label for="k-threshold">Keys Required (k):</label>
          <input 
            id="k-threshold" 
            v-model.number="kThreshold" 
            type="number" 
            min="1" 
            :max="nShares"
            class="number-input"
          />
        </div>
        <div class="input-group">
          <label for="n-shares">Total Shares (n):</label>
          <input 
            id="n-shares" 
            v-model.number="nShares" 
            type="number" 
            min="1" 
            max="10"
            class="number-input"
          />
        </div>
        <p class="config-description">
          {{ kThreshold }} of {{ nShares }} shares will be required for decryption
        </p>
      </div>
    </div>

    <div class="section">
      <h2>Encryption</h2>
      <button 
        @click="encryptFile" 
        :disabled="isEncrypting || !selectedFile || !outputDir" 
        class="btn btn-primary"
      >
        {{ isEncrypting ? "Encrypting..." : "Encrypt File" }}
      </button>
      
      <div v-if="shares.length > 0" class="shares-output">
        <h3>Generated Shares (save these securely!):</h3>
        <div class="shares-list">
          <div v-for="(share, index) in shares" :key="index" class="share-item">
            <label>Share {{ index + 1 }}:</label>
                         <input :value="share" readonly class="share-input" @click="($event.target as HTMLInputElement)?.select()" />
          </div>
        </div>
      </div>
    </div>

    <div class="section">
      <h2>Decryption</h2>
      <div class="input-group">
        <label for="input-shares">Enter Shares (one per line):</label>
        <textarea 
          id="input-shares"
          v-model="inputShares" 
          rows="4" 
          placeholder="Paste your shares here, one per line"
          class="shares-textarea"
        ></textarea>
      </div>
      
      <button 
        @click="decryptFile" 
        :disabled="isDecrypting || !selectedFile || !outputDir || !inputShares" 
        class="btn btn-primary"
      >
        {{ isDecrypting ? "Decrypting..." : "Decrypt File" }}
      </button>
    </div>

    <div v-if="result" class="result">
      <h3>Result:</h3>
      <p>{{ result }}</p>
    </div>
  </main>
</template>

<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";

const selectedFile = ref<string>("");
const outputDir = ref<string>("");
const kThreshold = ref<number>(2);
const nShares = ref<number>(3);
const shares = ref<string[]>([]);
const inputShares = ref<string>("");
const isEncrypting = ref<boolean>(false);
const isDecrypting = ref<boolean>(false);
const result = ref<string>("");

async function selectFile() {
  try {
    console.log("Opening file dialog...");
    const file = await open({
      multiple: false,
      directory: false,
      title: "Select file to encrypt/decrypt"
    });
    console.log("File dialog result:", file);
    
    if (file && typeof file === "string") {
      selectedFile.value = file;
      console.log("File selected:", file);
    } else if (file === null) {
      console.log("File selection cancelled by user");
    }
  } catch (error) {
    console.error("Error selecting file:", error);
    result.value = `Error selecting file: ${error}`;
  }
}

async function selectOutputDir() {
  try {
    console.log("Opening directory dialog...");
    const dir = await open({
      multiple: false,
      directory: true,
      title: "Select output directory"
    });
    console.log("Directory dialog result:", dir);
    
    if (dir && typeof dir === "string") {
      outputDir.value = dir;
      console.log("Directory selected:", dir);
    } else if (dir === null) {
      console.log("Directory selection cancelled by user");
    }
  } catch (error) {
    console.error("Error selecting directory:", error);
    result.value = `Error selecting directory: ${error}`;
  }
}

async function encryptFile() {
  if (!selectedFile.value || !outputDir.value) {
    alert("Please select a file and output directory");
    return;
  }
  
  isEncrypting.value = true;
  try {
    const response = await invoke("encrypt_file", {
      filePath: selectedFile.value,
      outputDir: outputDir.value,
      k: kThreshold.value,
      n: nShares.value
    }) as { shares: string[] };
    
    shares.value = response.shares;
    result.value = `File encrypted successfully! Shares generated.`;
  } catch (error) {
    result.value = `Encryption failed: ${error}`;
  } finally {
    isEncrypting.value = false;
  }
}

async function decryptFile() {
  if (!selectedFile.value || !outputDir.value || !inputShares.value) {
    alert("Please select a file, output directory, and provide shares");
    return;
  }
  
  isDecrypting.value = true;
  try {
    const sharesArray = inputShares.value.split('\n').filter(s => s.trim());
    const response = await invoke("decrypt_file", {
      filePath: selectedFile.value,
      outputDir: outputDir.value,
      shares: sharesArray
    }) as { outputPath: string };
    
    result.value = `File decrypted successfully to: ${response.outputPath}`;
  } catch (error) {
    result.value = `Decryption failed: ${error}`;
  } finally {
    isDecrypting.value = false;
  }
}
</script>

<style scoped>
.container {
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', 'Roboto', sans-serif;
}

h1 {
  color: #2c3e50;
  margin-bottom: 30px;
  text-align: center;
}

.section {
  background: #f8f9fa;
  border-radius: 8px;
  padding: 20px;
  margin-bottom: 20px;
  border: 1px solid #e9ecef;
}

h2 {
  color: #495057;
  margin-top: 0;
  margin-bottom: 15px;
  font-size: 1.2em;
}

.file-input {
  display: flex;
  align-items: center;
  gap: 15px;
  margin-bottom: 10px;
}

.file-path {
  color: #6c757d;
  font-style: italic;
}

.sss-config {
  display: flex;
  flex-direction: column;
  gap: 15px;
}

.input-group {
  display: flex;
  flex-direction: column;
  gap: 5px;
}

.input-group label {
  font-weight: 500;
  color: #495057;
}

.number-input {
  width: 100px;
  padding: 8px 12px;
  border: 1px solid #ced4da;
  border-radius: 4px;
  font-size: 14px;
}

.config-description {
  color: #6c757d;
  font-style: italic;
  margin: 0;
}

.btn {
  padding: 10px 20px;
  border: none;
  border-radius: 4px;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: background-color 0.2s;
}

.btn:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.btn-primary {
  background-color: #007bff;
  color: white;
}

.btn-primary:hover:not(:disabled) {
  background-color: #0056b3;
}

.btn-secondary {
  background-color: #6c757d;
  color: white;
}

.btn-secondary:hover:not(:disabled) {
  background-color: #545b62;
}

.shares-output {
  margin-top: 20px;
  padding: 15px;
  background: #e7f3ff;
  border: 1px solid #b3d9ff;
  border-radius: 4px;
}

.shares-list {
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.share-item {
  display: flex;
  flex-direction: column;
  gap: 5px;
}

.share-item label {
  font-weight: 500;
  color: #495057;
}

.share-input {
  padding: 8px 12px;
  border: 1px solid #ced4da;
  border-radius: 4px;
  font-family: monospace;
  font-size: 12px;
  background: white;
  cursor: pointer;
}

.shares-textarea {
  width: 100%;
  padding: 10px;
  border: 1px solid #ced4da;
  border-radius: 4px;
  font-family: monospace;
  font-size: 12px;
  resize: vertical;
}

.result {
  background: #d1ecf1;
  border: 1px solid #bee5eb;
  border-radius: 4px;
  padding: 15px;
  margin-top: 20px;
}

.result h3 {
  margin-top: 0;
  color: #0c5460;
}

.result p {
  margin: 0;
  color: #0c5460;
}

@media (prefers-color-scheme: dark) {
  .container {
    color: #f8f9fa;
  }
  
  .section {
    background: #343a40;
    border-color: #495057;
  }
  
  h1, h2 {
    color: #f8f9fa;
  }
  
  .number-input, .share-input, .shares-textarea {
    background: #495057;
    border-color: #6c757d;
    color: #f8f9fa;
  }
  
  .shares-output {
    background: #1a4d66;
    border-color: #2980b9;
  }
  
  .result {
    background: #155724;
    border-color: #28a745;
  }
}
</style>