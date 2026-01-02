<script>
  import { invoke } from "@tauri-apps/api/core";
  
  let activeTab = 'project'; 
  let projectName = "Untitled Path";
  let materialWidth = 300;
  let materialHeight = 200;

  const tabs = [
    { id: 'project', icon: '📁', label: 'Project' },
    { id: 'import',  icon: '📥', label: 'Import' },
    { id: 'machine', icon: '⚙️', label: 'Machine' },
    { id: 'preview', icon: '👁️', label: 'Preview' },
    { id: 'settings', icon: '🛠️', label: 'Settings' }
  ];
</script>

<div class="app-layout">
  <nav class="sidebar">
    <div class="logo">PS</div>
    {#each tabs as tab}
      <button 
        class:active={activeTab === tab.id} 
        on:click={() => activeTab = tab.id}
        title={tab.label}
      >
        <span class="icon">{tab.icon}</span>
        <span class="label">{tab.label}</span>
      </button>
    {/each}
  </nav>

  <main class="workspace">
    <header class="top-bar">
      <div class="project-info">
        <span class="label-text">Project:</span>
        <input bind:value={projectName} placeholder="Enter project name..." />
      </div>
      <div class="actions">
        <button class="btn-primary" on:click={() => console.log('Saving...')}>
          Save Path
        </button>
      </div>
    </header>

    <section class="canvas-area">
      {#if activeTab === 'project'}
        <div class="panel">
          <h2>Project Setup</h2>
          <p>Define your material size and origin point.</p>
          <div class="setup-grid">
             <div class="input-group">
               <label for="width-input">Width (mm)</label>
               <input id="width-input" type="number" bind:value={materialWidth} />
             </div>
             <div class="input-group">
               <label for="height-input">Height (mm)</label>
               <input id="height-input" type="number" bind:value={materialHeight} />
             </div>
          </div>
        </div>
      {/if}

      <div class="grid-viewer">
         <div class="origin-point"></div>
         <p style="color: #444; font-size: 0.8rem; user-select: none;">CNC WORK AREA</p>
      </div>
    </section>
  </main>
</div>

<style>
  /* 1. FORCE THE WINDOW TO BE DARK AND FULL SCREEN */
  :global(html, body) {
    margin: 0 !important;
    padding: 0 !important;
    width: 100vw !important;
    height: 100vh !important;
    overflow: hidden !important;
    background-color: #0f0f0f !important;
    color: #e0e0e0;
    font-family: sans-serif;
  }

  /* 2. THE MAIN CONTAINER - Pinned to the top-left corner */
  .app-layout {
    position: fixed; /* Bypasses any 'center' divs from the template */
    top: 0;
    left: 0;
    width: 100vw;
    height: 100vh;
    display: flex;
    background: #0f0f0f;
  }

  /* 3. SIDEBAR - Fixed Width */
  .sidebar {
    width: 72px;
    height: 100%;
    background: #181818;
    border-right: 1px solid #2a2a2a;
    display: flex;
    flex-direction: column;
    align-items: center;
    padding-top: 20px;
    flex-shrink: 0;
  }

  /* 4. WORKSPACE - Grows to fill the rest */
  .workspace {
    flex-grow: 1;
    display: flex;
    flex-direction: column;
    height: 100%;
    min-width: 0;
  }

  .top-bar {
    height: 48px;
    background: #1a1a1a;
    border-bottom: 1px solid #2a2a2a;
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0 20px;
  }

  .canvas-area {
    flex-grow: 1;
    display: flex;
    flex-direction: column;
    padding: 15px;
    gap: 15px;
    overflow: hidden;
  }

  .grid-viewer {
    flex-grow: 1;
    background-color: #000;
    border: 1px solid #333;
    border-radius: 4px;
    position: relative;
    /* CNC Grid background */
    background-image: 
      linear-gradient(#1a1a1a 1px, transparent 1px),
      linear-gradient(90deg, #1a1a1a 1px, transparent 1px);
    background-size: 20px 20px;
  }

  /* SIDEBAR UI */
  .logo { color: #646cff; font-weight: bold; margin-bottom: 20px; }
  
  .sidebar button {
    background: none; border: none; color: #555;
    width: 60px; height: 60px; cursor: pointer;
    display: flex; flex-direction: column; align-items: center; justify-content: center;
  }

  .sidebar button.active { color: #646cff; background: #222; border-radius: 8px; }

  .icon { font-size: 1.5rem; }
  .label { font-size: 0.6rem; margin-top: 4px; }

  /* WORKSPACE UI */
  .project-info { display: flex; align-items: center; gap: 8px; }
  input { background: #252525; border: 1px solid #444; color: white; padding: 4px 8px; border-radius: 4px; }
  .btn-primary { background: #646cff; border: none; color: white; padding: 6px 12px; border-radius: 4px; cursor: pointer; }
</style>