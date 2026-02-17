const { invoke } = window.__TAURI__.core;

// --- Pre-fill output directories with Downloads ---
(async function initDefaults() {
  try {
    const dl = await invoke('get_downloads_dir');
    document.getElementById('extract-dir').value = dl;
    document.getElementById('compress-dir').value = dl;
    document.getElementById('convert-dir').value = dl;
  } catch (_) {}
})();

// --- Tabs ---
document.querySelectorAll('.tab').forEach(tab => {
  tab.addEventListener('click', () => {
    document.querySelectorAll('.tab').forEach(t => t.classList.remove('active'));
    document.querySelectorAll('.panel').forEach(p => p.classList.remove('active'));
    tab.classList.add('active');
    document.getElementById(tab.dataset.tab).classList.add('active');
    hideStatus();
  });
});

// --- File pickers (via Rust invoke, bound with addEventListener) ---
document.querySelectorAll('.btn-browse').forEach(btn => {
  btn.addEventListener('click', async () => {
    const target = btn.dataset.target;
    const kind = btn.dataset.pick;  // "file" or "dir"
    try {
      let path;
      if (kind === 'file') {
        path = await invoke('pick_file');
      } else if (kind === 'dir') {
        path = await invoke('pick_directory');
      }
      if (path) document.getElementById(target).value = path;
    } catch (e) {
      // User cancelled – ignore
      console.log('picker cancelled or error:', e);
    }
  });
});

// --- Extract mode toggle ---
document.getElementById('extract-mode').addEventListener('change', (e) => {
  const mode = e.target.value;
  const pagesField = document.getElementById('extract-pages-field');
  const nameHint = document.getElementById('extract-name-hint');
  const actionBtn = document.querySelector('#extract .btn-action');

  if (mode === 'text') {
    pagesField.style.display = 'none';
    nameHint.textContent = '(optional — .txt added automatically)';
    actionBtn.textContent = 'Extract Text';
  } else {
    pagesField.style.display = '';
    nameHint.textContent = '(optional — .pdf added automatically)';
    actionBtn.textContent = 'Extract Pages';
  }
});

// --- Action buttons ---
document.querySelectorAll('.btn-action').forEach(btn => {
  btn.addEventListener('click', () => {
    const action = btn.dataset.action;
    if (action === 'extract') {
      const mode = document.getElementById('extract-mode').value;
      if (mode === 'text') runExtractText();
      else runExtract();
    }
    else if (action === 'compress') runCompress();
    else if (action === 'convert') runConvert();
  });
});

// --- Status ---
function showStatus(message, type) {
  const el = document.getElementById('status');
  el.textContent = message;
  el.className = `status ${type}`;
}

function hideStatus() {
  document.getElementById('status').className = 'status hidden';
}

function setLoading(btn) {
  btn.disabled = true;
  btn.dataset.originalText = btn.textContent;
  btn.textContent = 'Processing...';
  showStatus('Processing...', 'loading');
}

function clearLoading(btn) {
  btn.disabled = false;
  btn.textContent = btn.dataset.originalText;
}

// --- Commands ---
async function runExtract() {
  const input = document.getElementById('extract-input').value;
  const pages = document.getElementById('extract-pages').value;
  const dir = document.getElementById('extract-dir').value;
  const name = document.getElementById('extract-name').value.trim();
  const btn = document.querySelector('#extract .btn-action');

  if (!input) return showStatus('Please select an input PDF file.', 'error');
  if (!pages) return showStatus('Please enter a page range.', 'error');

  setLoading(btn);
  try {
    const result = await invoke('cmd_extract', { input, pages, outputDir: dir, outputName: name });
    showStatus(result, 'success');
  } catch (e) {
    showStatus(e, 'error');
  }
  clearLoading(btn);
}

async function runExtractText() {
  const input = document.getElementById('extract-input').value;
  const dir = document.getElementById('extract-dir').value;
  const name = document.getElementById('extract-name').value.trim();
  const btn = document.querySelector('#extract .btn-action');

  if (!input) return showStatus('Please select an input PDF file.', 'error');

  setLoading(btn);
  try {
    const result = await invoke('cmd_extract_text', { input, outputDir: dir, outputName: name });
    showStatus(result, 'success');
  } catch (e) {
    showStatus(e, 'error');
  }
  clearLoading(btn);
}

async function runCompress() {
  const input = document.getElementById('compress-input').value;
  const quality = document.getElementById('compress-quality').value;
  const dir = document.getElementById('compress-dir').value;
  const name = document.getElementById('compress-name').value.trim();
  const btn = document.querySelector('#compress .btn-action');

  if (!input) return showStatus('Please select an input PDF file.', 'error');

  setLoading(btn);
  try {
    const result = await invoke('cmd_compress', { input, quality, outputDir: dir, outputName: name });
    showStatus(result, 'success');
  } catch (e) {
    showStatus(e, 'error');
  }
  clearLoading(btn);
}

async function runConvert() {
  const input = document.getElementById('convert-input').value;
  const format = document.getElementById('convert-format').value;
  const dpi = parseInt(document.getElementById('convert-dpi').value) || 300;
  const dir = document.getElementById('convert-dir').value;
  const btn = document.querySelector('#convert .btn-action');

  if (!input) return showStatus('Please select an input PDF file.', 'error');

  setLoading(btn);
  try {
    const result = await invoke('cmd_convert', { input, format, dpi, outputDir: dir });
    showStatus(result, 'success');
  } catch (e) {
    showStatus(e, 'error');
  }
  clearLoading(btn);
}
