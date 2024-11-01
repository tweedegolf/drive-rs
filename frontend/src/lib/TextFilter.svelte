<script lang="ts">
    import type {FullCrate} from "../crate-db";
    import {sort_by, scores} from '../store/FilterStore.ts';
    import Fuse from "fuse.js";

    export let crates: FullCrate[];
    export let selected: number[][];

    interface Score {
      name: String;
      score: number;
    }

    let search: string = "";

    const options = {
        keys: [
            "description",
            "manufacturer",
            {
                name: "name",
                weight: 10,
            },
            "names",
            "part_numbers",
            "resources.title",
        ],
        includeMatches: true,
        includeScore: true,
        useExtendedSearch: false,
    };
    const index = Fuse.createIndex(options.keys, crates);
    const fuse = new Fuse(crates, options, index);

    // Store the scores in the FilterStore on changes
    // and force sorting by score
    $ : {
      if (search.length > 0) {
        $sort_by = 'score';
        $scores = (fuse.search(search).map((r) => ({ score: r.score, name: r.item.name } as Score))) as Score[];
      } else {
        $sort_by = 'alphanumeric';
      }
    }

    $: results = fuse.search(search);
    $: selected = search === "" ? [] : [results.map((r) => r.refIndex)];
</script>

<div>
    <input type="text" placeholder="Search" bind:value={search}>
</div>
