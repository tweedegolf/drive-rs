<script lang="ts">
    import type {FullCrate} from "../crate-db";
    import Crate from "./Crate.svelte";
    import {open_filter} from '../store/FilterStore.svelte';

    export let crates: FullCrate[];

    export let filter: number[];
    export let cols_shown: string[] = ["name", "description", "downloads", "updated_at"];

    let filtered_crates: FullCrate[] = crates;
</script>

<ol on:click={() => $open_filter = ""}>
{#each filtered_crates as crate, i}
    {#if filter.includes(i)}
        <li><Crate crate={crate}/></li>
    {/if}
{/each}
</ol>

<style>
    ol {
        list-style-type: none;
        padding: 0;
    }
</style>
