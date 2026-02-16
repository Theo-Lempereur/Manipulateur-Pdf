const { invoke } = window.__TAURI__.core;
const { open, save } = window.__TAURI__.dialog;

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

// --- File pickers ---
async function pickFile(inputId) {
  const path = await open({
    filters: [{ name: 'PDF', extensions: ['pdf'] }],
    multiple: false,
  });
  if (path) {
    document.getElementById(inputId).value = path;
  }
}

async function pickSave(inputId, ext) {
  const path = await save({
    filters: [{ name: ext.toUpperCase(), extensions: [ext] }],
  });
  if (path) {
    document.getElementById(inputId).value = path;
  }
}

async function pickDir(inputId) {
  const path = await open({ directory: true });
  if (path) {
    document.getElementById(inputId).value = path;
  }
}

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
  const output = document.getElementById('extract-output').value;
  const btn = document.querySelector('#extract .btn-action');

  if (!input) return showStatus('Please select an input PDF file.', 'error');
  if (!pages) return showStatus('Please enter a page range.', 'error');

  setLoading(btn);
  try {
    const result = await invoke('cmd_extract', { input, pages, output });
    showStatus(result, 'success');
  } catch (e) {
    showStatus(e, 'error');
  }
  clearLoading(btn);
}

async function runCompress() {
  const input = document.getElementById('compress-input').value;
  const quality = document.getElementById('compress-quality').value;
  const output = document.getElementById('compress-output').value;
  const btn = document.querySelector('#compress .btn-action');

  if (!input) return showStatus('Please select an input PDF file.', 'error');

  setLoading(btn);
  try {
    const result = await invoke('cmd_compress', { input, quality, output });
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
  const output = document.getElementById('convert-output').value;
  const btn = document.querySelector('#convert .btn-action');

  if (!input) return showStatus('Please select an input PDF file.', 'error');

  setLoading(btn);
  try {
    const result = await invoke('cmd_convert', { input, format, dpi, output });
    showStatus(result, 'success');
  } catch (e) {
    showStatus(e, 'error');
  }
  clearLoading(btn);
}
