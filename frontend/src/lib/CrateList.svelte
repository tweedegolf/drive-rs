<script lang="ts">
  import type { FullCrate } from "../crate-db";
  import Crate from "./Crate.svelte";
  import { open_filter, sort_by, scores } from "../store/FilterStore.ts";

  interface Score {
    name: String;
    score: number;
  }

  export let crates: FullCrate[];

  export let filter: number[];

  let filtered_crates: FullCrate[];

  function compare(a: string | number, b: string | number): number {
    if (a < b) {
      return -1;
    }
    if (a > b) {
      return 1;
    }
    return 0;
  }

  $: filtered_crates = crates
    .filter((_, i) => filter.includes(i))
    .toSorted((a, b) => {
      if ($sort_by == "all_downloads") {
        return compare(b.downloads, a.downloads);
      } else if ($sort_by == "recent_downloads") {
        return compare(b.this_version_downloads, a.this_version_downloads);
      } else if ($sort_by == "newly_added") {
        return compare(b.created_at, a.created_at);
      } else if ($sort_by == "recently_updated") {
        return compare(b.updated_at, a.updated_at);
      }

      // Sort by text search score
      else if ($sort_by == "score") {
        const score_a = $scores.find((s: Score) => s.name == a.name);
        const score_b = $scores.find((s: Score) => s.name == b.name);
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

      return compare(a.name, b.name);
    });
</script>

<ol>
  {#each filtered_crates as crate, i}
    <li>
      <Crate {crate} />
    </li>
  {/each}
</ol>

<style>
  ol {
    list-style-type: none;
    padding: 0;
  }
</style>
