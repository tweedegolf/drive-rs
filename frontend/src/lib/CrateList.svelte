<script lang="ts">
    import type {FullCrate} from "../crate-db";
    import Crate from "./Crate.svelte";
    import {open_filter, sort_by, scores} from '../store/FilterStore.ts';

    interface Score {
      name: String;
      score: number;
    }

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
        return b.created_at > a.created_at ? 1 : 0;
      }
      else if ($sort_by == 'recently_updated') {
        return b.updated_at > a.updated_at ? 1 : 0;
      }

      // Sort by text search score
      else if ($sort_by == 'score') {
        const score_a = $scores.find((s : Score) => s.name == a.name);
        const score_b = $scores.find((s : Score) => s.name == b.name);
        if (score_a && score_b) {
          return score_a.score - score_b.score;
        } else if (score_a) {
          return 0 - score_a.score;
        } else if (score_b) {
          return score_b.score;
        } else {
          return 0;
        }
      }

      return b.name < a.name ? 1 : 0;
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
