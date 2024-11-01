<script lang="ts">
  import type { FullCrate } from "../crate-db";

  export let crate: FullCrate;

  let showNotification = false;
  let showKiCadSymbol = false;
  let showDevBoards = false;

  function copyToClipboard() {
    navigator.clipboard.writeText(`${crate.name} = "${crate.version}"`);
    showNotification = true;
    setTimeout(() => {
      showNotification = false;
    }, 2000); // Hide notification after 2 seconds
  }

  function toggleKiCadSymbol() {
    showKiCadSymbol = !showKiCadSymbol;
  }

  function toggleDevBoards() {
    showDevBoards = !showDevBoards;
  }
</script>

<div class="card">
  <div class="description-box">
    <div class="crate-title">
      <a class="crate-title" href="https://crates.io/crates/{crate.name}"
        >{crate.name}</a
      >
      <span>{crate.version}</span>
      <button
        title="Copy Cargo.toml snippet to clipboard"
        on:click={copyToClipboard}
        >📋
        {#if showNotification}Copied!{/if}
      </button>
    </div>
    <div class="links">
      {#if crate.documentation}<span
          ><a href={crate.documentation}>📚 Docs</a></span
        >{/if}
      {#if crate.repository}<span><a href={crate.repository}>🗃️ Repo</a></span
        >{/if}
      {#if crate.homepage}<span><a href={crate.homepage}>🏠 Homepage</a></span
        >{/if}
    </div>
    {#if crate.names.length > 0 || crate.part_numbers}
      <div>
        <span>🕷️ Supported chips: {crate.names.join(", ")}</span>
      </div>
    {/if}
    <div class="description">
      {crate.description}
    </div>
    <ul class="categories">
      {#each crate.categories || [] as category}
        <li>{category}</li>
      {/each}
    </ul>
  </div>

  <div class="tags-box">
    {#if crate.dependencies.includes("embedded-hal@1.0.0")}
      <p title="Supports embedded-hal 1.0">✅ e-h 1.0</p>
    {/if}
    {#if crate.dependencies.includes("embedded-hal-async@1.0.0")}
      <p title="Supports embedded-hal-async 1.0">✅ e-h-async 1.0</p>
    {/if}
    {#if crate.manufacturer !== "Unknown"}
      <p>🛠️ Manufacturer: {crate.manufacturer}</p>
    {/if}
    {#if crate.interfaces && (crate.interfaces.i2c || crate.interfaces.spi)}
      <p>
        🚌 Interfaces:
        {#if crate.interfaces.i2c}I2C{/if}
        {#if crate.interfaces.spi}SPI{/if}
      </p>
    {/if}
    {#if crate.rust_version}
      <p>🛠️ MSRV: {crate.rust_version}</p>
    {/if}
    {#if crate.datasheets}
      <p>📋 <a href={crate.datasheets[0]}>Datasheet</a></p>
      <!-- TODO: Handle multiple datasheets -->
    {/if}
    {#if crate.dev_boards}
      <p>
        <button on:click={toggleDevBoards} aria-expanded={showDevBoards}
          >✅ Dev Board
        </button>
        {#if showDevBoards}
          <ul>
            {#each crate.dev_boards || [] as board}
              <li><a href={board.link} target="_blank">{board.name}</a></li>
            {/each}
          </ul>
        {/if}
      </p>
    {/if}
    {#if crate.kicad_symbol}
      <p>
        <button on:click={toggleKiCadSymbol} aria-expanded={showKiCadSymbol}>
          ✅ KiCad {crate.kicad_symbol.length > 1 ? "Symbols" : "Symbol"}
        </button>
        {#if showKiCadSymbol}
          <ul>
            {#each crate.kicad_symbol as sym}
              <li>{sym}</li>
            {/each}
          </ul>
        {/if}
      </p>
    {/if}
    {#if crate.packages}
      <p>👣 Footprints: {crate.packages.join(", ")}</p>
    {/if}
    {#if crate.resources}
      <div>
        🗞️ Resources:
        <ul>
          {#each crate.resources as resource}
            <li><a href={resource.link}>{resource.title}</a></li>
          {/each}
        </ul>
      </div>
    {/if}
  </div>
  <div class="stats-box">
    <p>👮 License: {crate.license}</p>
    <p>⬇️ All-Time: {crate.downloads}</p>
    <p>⬇️ This version: {crate.this_version_downloads}</p>
    <p>
      <span>🔄 Last updated: </span>
      <time datetime={crate.updated_at} class="ember-tooltip-target">
        {crate.updated_at.substring(0, 10)}
        <!-- TODO: Make this nicer -->
      </time>
    </p>
    {#if crate.crate_size}
      <p>🏋️ Size: {(crate.crate_size / 1024).toFixed(1)} kB</p>
    {/if}
  </div>
</div>

<style>
  .categories {
    display: flex;
    flex-direction: row;
    gap: 8px;
    list-style-type: none;
    margin: 0;
    padding: 0;
  }

  .categories li {
    background-color: var(--color-tertiary);
    border: 1px solid var(--color-border);
    border-radius: var(--radius);
    padding: 4px 12px;
  }

  .card {
    text-align: left;
    display: flex;
    border: 1px solid #ccc;
    padding: 16px;
    margin: 8px 0;
    border-radius: var(--radius);
    background-color: var(--color-background);
    width: 100%;
    box-sizing: border-box;
  }

  .description-box,
  .stats-box,
  .tags-box {
    flex: 1;
    margin: 0 8px;
  }

  .description-box {
    flex: 3;
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  .description {
    flex-grow: 1;
    max-width: 90%;
  }

  .links {
    display: flex;
    gap: 16px;
  }

  .crate-title {
    font-weight: 600;
    display: flex;
    align-items: baseline;
    gap: 8px;
  }

  .crate-title a {
    font-size: 20px;
  }

  .tags-box,
  .stats-box {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .tags-box p,
  .stats-box p {
    margin: 0;
    padding: 4px 0;
  }
</style>
