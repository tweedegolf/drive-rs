<script lang="ts">
    import type {FullCrate} from "../crate-db";

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


<div class="crate-box">
    <div class="description-box">
        <div>
            <a href="https://crates.io/crates/{crate.name}">{crate.name}</a>
            <span>{crate.version}</span>
            <button title="Copy Cargo.toml snippet to clipboard" on:click={copyToClipboard}>📋
                {#if showNotification}Copied!{/if}
            </button>
            {#if crate.documentation}<span><a href="{crate.documentation}">📚 Docs</a></span>{/if}
            {#if crate.repository}<span><a href="{crate.repository}">🗃️ Repo</a></span>{/if}
            {#if crate.homepage}<span><a href="{crate.homepage}">🏠 Homepage</a></span>{/if}
        </div>
        {#if crate.names.length > 0 || crate.part_numbers }
            <div>
                <span>🕷️ Supported chips: {crate.names.join(", ")}</span>
            </div>
        {/if}
        <div>
            {crate.description}
        </div>
        <ul>
            {#each crate.categories || [] as category}
                <li>{category}</li>
            {/each}
        </ul>
    </div>
    <div class="tags-box">
        {#if crate.manufacturer !== "Unknown" }
            <div>🛠️ Manufacturer: {crate.manufacturer}</div>
        {/if}
        {#if crate.interfaces && (crate.interfaces.i2c || crate.interfaces.spi) }
            <div>🚌 Interfaces:
                {#if crate.interfaces.i2c}I2C{/if}
                {#if crate.interfaces.spi}SPI{/if}
            </div>
        {/if}
        {#if crate.rust_version }
            <div>🛠️ MSRV: {crate.rust_version}</div>
        {/if}
        {#if crate.datasheets }
            <div>📋 <a href="{crate.datasheets[0]}">Datasheet</a></div> <!-- TODO: Handle multiple datasheets -->
        {/if}
        {#if crate.dev_boards }
            <div>
                <button on:click={toggleDevBoards} aria-expanded="{showDevBoards}">✅ Dev Board</button>
                {#if showDevBoards}
                    <ul>
                        {#if crate.dev_boards.adafruit}
                            <li><a href="https://www.adafruit.com/product/{crate.dev_boards.adafruit}">
                                Adafruit-{crate.dev_boards.adafruit}
                            </a></li>
                        {/if}
                        {#if crate.dev_boards.mikroe}
                            <li><a href="https://www.mikroe.com/search?search_query=MIKROE-{crate.dev_boards.mikroe}">
                                MIKROE-{crate.dev_boards.mikroe}
                            </a></li>
                        {/if}
                        {#if crate.dev_boards.sparkfun}
                            <li><a href="https://www.sparkfun.com/products/{crate.dev_boards.sparkfun}">
                                SparkFun-{crate.dev_boards.sparkfun}
                            </a></li>
                        {/if}
                        {#each crate.dev_boards.other || [] as board}
                            <li><a href="{board.link}">{board.name}</a></li>
                        {/each}
                    </ul>
                {/if}
            </div>
        {/if}
        {#if crate.kicad_symbol}
            <div>
                <button on:click={toggleKiCadSymbol} aria-expanded="{showKiCadSymbol}">
                    ✅ KiCad {crate.kicad_symbol.length > 1 ? "Symbols" : "Symbol"}
                </button>
                {#if showKiCadSymbol}
                    <ul>
                        {#each crate.kicad_symbol as sym}
                            <li>{sym}</li>
                        {/each}
                    </ul>
                {/if}
            </div>
        {/if}
        {#if crate.packages}
            <div>👣 Footprints: {crate.packages.join(", ")}</div>
        {/if}
        {#if crate.resources}
            <div>
                🗞️ Resources:
                <ul>
                    {#each crate.resources as resource}
                        <li><a href="{resource.link}">{resource.title}</a></li>
                    {/each}
                </ul>
            </div>
        {/if}
    </div>
    <div class="stats-box">
        <div>👮 License: {crate.license}</div>
        <div> ⬇️ All-Time: {crate.downloads} </div>
        <div> ⬇️ This version: {crate.this_version_downloads} </div>
        <div>
            <span>🔄 Last updated: </span>
            <time datetime="{crate.updated_at}" class="ember-tooltip-target">
                {crate.updated_at.substring(0, 10)} <!-- TODO: Make this nicer -->
            </time>
        </div>
        {#if crate.crate_size}
            <div>🏋️ Size: {(crate.crate_size / 1024).toFixed(1)} kB</div>
        {/if}
    </div>
</div>

<style>
    .crate-box {
        text-align: left;
        display: flex;
        justify-content: space-between;
        border: 1px solid #ccc;
        padding: 16px;
        margin: 8px 0;
        border-radius: 8px;
        background-color: var(--color-background);
        width: 100%;
        box-sizing: border-box;
    }

    .description-box, .stats-box, .tags-box {
        flex: 1;
        margin: 0 8px;
    }

    .description-box {
        flex: 3;
    }

    .description-box {
        max-width: 70%;
    }

    .stats-box {
        max-width: 30%;
        display: flex;
        flex-direction: column;
        justify-content: space-around;
    }

    .tags-box div, .stats-box div {
        background: var(--color-tertiary);
        border-radius: 2px;
        margin: 4px 0;
    }
</style>
