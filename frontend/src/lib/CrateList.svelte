<script lang="ts">
    import type {FullCrate} from "../crate-db";
    import Crate from "./Crate.svelte";
    import {open_filter, sort_by} from '../store/FilterStore.svelte';

    export let crates: FullCrate[];

    export let filter: number[];

    let filtered_crates: FullCrate[];

    $ : filtered_crates = crates.filter((_, i) => filter.includes(i)).sort((a, b) => {
      if ($sort_by == 'all_downloads') {
        return b.downloads - a.downloads;
      }
      else if ($sort_by == 'recent_downloads') {
        return b.this_version_downloads - a.this_version_downloads;
      }
      else if ($sort_by == 'newly_added') {
        return b.created_at > a.created_at;
      }
      else if ($sort_by == 'recently_updated') {
        return b.updated_at > a.updated_at;
      }
      return b.name < a.name;
    });
</script>

<ol>
    {#each filtered_crates as crate, i}
       <li>
         <Crate crate={crate}/>
       </li>
    {/each}
</ol>

<style>
    ol {
        list-style-type: none;
        padding: 0;
    }
</style>
